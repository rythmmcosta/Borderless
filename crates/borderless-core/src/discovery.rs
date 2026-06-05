//! LAN device discovery via mDNS-SD.
//!
//! Advertises this device as `_borderless._tcp.local.` and browses for peers.
//! When a peer is found it is cached in the `peers` map.
//! The engine uses this cache to initiate direct LAN connections.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{CoreError, identity::DeviceIdentity};

const SERVICE_TYPE: &str = "_borderless._tcp.local.";
const LISTEN_PORT:  u16  = 49152;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    pub id:       Uuid,
    pub name:     String,
    pub ip:       IpAddr,
    pub port:     u16,
    pub platform: String,
    pub version:  String,
    /// Hex-encoded X25519 DH public key (used to pre-verify before TCP handshake)
    pub dh_pubkey: String,
}

pub struct DiscoveryService {
    local_id:       Uuid,
    local_name:     String,
    local_dh_pubkey: String,
    peers:          Arc<RwLock<HashMap<Uuid, DiscoveredPeer>>>,
    running:        Arc<std::sync::atomic::AtomicBool>,
}

impl DiscoveryService {
    pub fn new(identity: &DeviceIdentity) -> Self {
        Self {
            local_id:        identity.id,
            local_name:      identity.display_name.clone(),
            local_dh_pubkey: hex::encode(identity.dh_public().as_bytes()),
            peers:           Arc::new(RwLock::new(HashMap::new())),
            running:         Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn start(&mut self) -> Result<(), CoreError> {
        use std::sync::atomic::Ordering;
        self.running.store(true, Ordering::SeqCst);

        let peers     = Arc::clone(&self.peers);
        let running   = Arc::clone(&self.running);
        let id        = self.local_id;
        let name      = self.local_name.clone();
        let dh_pubkey = self.local_dh_pubkey.clone();

        tokio::task::spawn_blocking(move || {
            mdns_loop(id, name, dh_pubkey, peers, running);
        });

        tracing::info!(service = SERVICE_TYPE, "LAN discovery started");
        Ok(())
    }

    pub fn stop(&self) {
        use std::sync::atomic::Ordering;
        self.running.store(false, Ordering::SeqCst);
    }

    /// Snapshot of all currently known LAN peers.
    pub fn peers(&self) -> Vec<DiscoveredPeer> {
        self.peers.read().unwrap().values().cloned().collect()
    }

    pub fn peer_by_id(&self, id: Uuid) -> Option<DiscoveredPeer> {
        self.peers.read().unwrap().get(&id).cloned()
    }
}

// ── mDNS loop (blocking, runs in spawn_blocking) ──────────────────

fn mdns_loop(
    id:        Uuid,
    name:      String,
    dh_pubkey: String,
    peers:     Arc<RwLock<HashMap<Uuid, DiscoveredPeer>>>,
    running:   Arc<std::sync::atomic::AtomicBool>,
) {
    use std::sync::atomic::Ordering;
    use std::time::Duration;
    use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

    let mdns = match ServiceDaemon::new() {
        Ok(d)  => d,
        Err(e) => { tracing::error!("mDNS daemon failed to start: {e}"); return; }
    };

    // Build a DNS-safe hostname
    let raw_host = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "borderless-device".to_string());
    let safe_host: String = raw_host
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect();
    let host_name = format!("{safe_host}.local.");

    // Determine outbound LAN IP
    let local_ip = local_ipv4()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "0.0.0.0".to_string());

    // TXT properties — short keys to stay well within the 255-byte TXT string limit
    let id_str   = id.to_string();
    let version  = env!("CARGO_PKG_VERSION");
    let platform = std::env::consts::OS;
    let props: &[(&str, &str)] = &[
        ("id", &id_str),
        ("nm", &name),
        ("pl", platform),
        ("v",  version),
        ("dh", &dh_pubkey),
    ];

    let service_info = match ServiceInfo::new(
        SERVICE_TYPE,
        &id_str,
        &host_name,
        local_ip.as_str(),
        LISTEN_PORT,
        props,
    ) {
        Ok(info) => info,
        Err(e)   => { tracing::error!("Failed to create mDNS service info: {e}"); return; }
    };

    if let Err(e) = mdns.register(service_info) {
        tracing::error!("Failed to register mDNS service: {e}");
        return;
    }

    let receiver = match mdns.browse(SERVICE_TYPE) {
        Ok(r)  => r,
        Err(e) => { tracing::error!("Failed to browse mDNS: {e}"); return; }
    };

    tracing::info!(%id, %name, ip = %local_ip, "mDNS advertising and browsing");

    while running.load(Ordering::SeqCst) {
        match receiver.recv_timeout(Duration::from_millis(200)) {
            Ok(event) => match event {
                ServiceEvent::ServiceResolved(info) => {
                    let props = info.get_properties();

                    // Extract and validate peer ID
                    let peer_id_str = props.get("id").map(|p| p.val_str().to_string());
                    let Some(peer_id_str) = peer_id_str else { continue; };
                    let Ok(peer_id) = Uuid::parse_str(&peer_id_str) else { continue; };
                    if peer_id == id { continue; } // skip ourselves

                    let addresses = info.get_addresses();
                    let Some(&peer_ipv4) = addresses.iter().next() else { continue; };

                    let peer = DiscoveredPeer {
                        id:        peer_id,
                        name:      props.get("nm").map(|p| p.val_str().to_string()).unwrap_or_default(),
                        ip:        IpAddr::V4(peer_ipv4),
                        port:      info.get_port(),
                        platform:  props.get("pl").map(|p| p.val_str().to_string()).unwrap_or_default(),
                        version:   props.get("v").map(|p| p.val_str().to_string()).unwrap_or_default(),
                        dh_pubkey: props.get("dh").map(|p| p.val_str().to_string()).unwrap_or_default(),
                    };

                    tracing::info!(
                        peer_id   = %peer_id,
                        ip        = %peer_ipv4,
                        port      = info.get_port(),
                        "Discovered peer on LAN"
                    );
                    peers.write().unwrap().insert(peer_id, peer);
                }

                ServiceEvent::ServiceRemoved(_, fullname) => {
                    peers.write().unwrap().retain(|k, _| !fullname.contains(&k.to_string()));
                    tracing::debug!(%fullname, "LAN peer removed");
                }

                _ => {}
            },

            Err(e) => {
                // flume::RecvTimeoutError — Timeout is normal, Disconnected means daemon stopped
                if format!("{e:?}").contains("Disconnected") {
                    tracing::warn!("mDNS channel disconnected");
                    break;
                }
                // Timeout — keep looping
            }
        }
    }

    // Unregister before exit
    let fullname = format!("{}.{}", id_str, SERVICE_TYPE);
    let _ = mdns.unregister(&fullname);
    tracing::info!("mDNS loop stopped");
}

/// Determine the primary outbound LAN IPv4 address without actually sending traffic.
fn local_ipv4() -> Option<std::net::Ipv4Addr> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:53").ok()?;
    match socket.local_addr().ok()? {
        std::net::SocketAddr::V4(a) => Some(*a.ip()),
        _                           => None,
    }
}
