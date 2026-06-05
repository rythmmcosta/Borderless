//! LAN device discovery via mDNS-SD.
//!
//! Advertises this device as `_borderless._tcp.local.` and browses for peers.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{CoreError, identity::DeviceIdentity};

const SERVICE_TYPE: &str = "_borderless._tcp.local.";
const LISTEN_PORT: u16    = 49152;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    pub id:          Uuid,
    pub name:        String,
    pub ip:          IpAddr,
    pub port:        u16,
    pub platform:    String,
    pub version:     String,
    pub dh_pubkey:   String,
}

pub struct DiscoveryService {
    local_id:   Uuid,
    local_name: String,
    peers:      Arc<RwLock<HashMap<Uuid, DiscoveredPeer>>>,
    running:    Arc<std::sync::atomic::AtomicBool>,
}

impl DiscoveryService {
    pub fn new(identity: &DeviceIdentity) -> Self {
        Self {
            local_id:   identity.id,
            local_name: identity.display_name.clone(),
            peers:      Arc::new(RwLock::new(HashMap::new())),
            running:    Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn start(&mut self) -> Result<(), CoreError> {
        use std::sync::atomic::Ordering;
        self.running.store(true, Ordering::SeqCst);
        let peers   = Arc::clone(&self.peers);
        let running = Arc::clone(&self.running);
        let id      = self.local_id;
        let name    = self.local_name.clone();
        tokio::task::spawn_blocking(move || { Self::mdns_loop(id, name, peers, running); });
        tracing::info!(service = SERVICE_TYPE, "LAN discovery started");
        Ok(())
    }

    pub fn stop(&self) {
        use std::sync::atomic::Ordering;
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn peers(&self) -> Vec<DiscoveredPeer> {
        self.peers.read().unwrap().values().cloned().collect()
    }

    pub fn peer_by_id(&self, id: Uuid) -> Option<DiscoveredPeer> {
        self.peers.read().unwrap().get(&id).cloned()
    }

    fn mdns_loop(
        id:      Uuid,
        name:    String,
        peers:   Arc<RwLock<HashMap<Uuid, DiscoveredPeer>>>,
        running: Arc<std::sync::atomic::AtomicBool>,
    ) {
        use std::sync::atomic::Ordering;
        // TODO: uncomment mdns-sd code block:
        //
        // let mdns = mdns_sd::ServiceDaemon::new().unwrap();
        // let info = mdns_sd::ServiceInfo::new(SERVICE_TYPE, &id.to_string(), &hostname,
        //     &local_ip, LISTEN_PORT, &[("id", id.to_string()), ("name", name.clone()),
        //       ("platform", std::env::consts::OS.to_string()),
        //       ("version", env!("CARGO_PKG_VERSION").to_string())]).unwrap();
        // mdns.register(info).unwrap();
        // let receiver = mdns.browse(SERVICE_TYPE).unwrap();
        // while running.load(Ordering::SeqCst) {
        //     if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
        //         match event {
        //             ServiceEvent::ServiceResolved(info) => { peers.write().unwrap().insert(peer_id, DiscoveredPeer { .. }); }
        //             ServiceEvent::ServiceRemoved(_, full_name) => { /* remove from map */ }
        //             _ => {}
        //         }
        //     }
        // }
        tracing::info!(%id, %name, "mDNS loop placeholder running");
        while running.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        tracing::info!("mDNS loop stopped");
    }
}
