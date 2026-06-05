//! Persistent device identity.
//!
//! Each device gets:
//!   - A stable UUID (derived from hardware fingerprint, survives reboots)
//!   - An Ed25519 keypair   (signing — proves "I am this device")
//!   - An X25519 keypair    (key exchange — establishes shared session keys)
//!
//! Stored in: `$CONFIG_DIR/borderless/identity.json`
//! TODO: encrypt with OS keychain (Keychain/Credential Store/libsecret)

use std::path::PathBuf;

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use x25519_dalek::{StaticSecret, PublicKey as X25519Public};
use uuid::Uuid;

use crate::error::CoreError;

/// Stable device identity — created once, persisted to disk.
#[derive(Clone)]
pub struct DeviceIdentity {
    /// Stable device UUID (consistent across reboots)
    pub id:           Uuid,
    /// Hardware fingerprint (hex SHA-256 of stable hardware IDs)
    pub fingerprint:  String,
    /// Human-readable name (hostname by default, user-changeable)
    pub display_name: String,
    /// Ed25519 signing key (proves identity in handshake)
    pub signing_key:  SigningKey,
    /// X25519 secret (ECDH key exchange)
    pub dh_secret:    StaticSecret,
}

impl DeviceIdentity {
    // ── Public key accessors ──────────────────────────────────────

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn dh_public(&self) -> X25519Public {
        X25519Public::from(&self.dh_secret)
    }

    /// Sign arbitrary data (used in handshake to prove device identity)
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.signing_key.sign(data)
    }

    /// ECDH key agreement + HKDF stretch → 32-byte session key material
    pub fn derive_session_key(&self, peer_dh_public: &X25519Public) -> [u8; 32] {
        let shared = self.dh_secret.diffie_hellman(peer_dh_public);
        let hk     = Hkdf::<Sha256>::new(None, shared.as_bytes());
        let mut okm = [0u8; 32];
        hk.expand(b"borderless-v1-session-key", &mut okm)
          .expect("HKDF length valid");
        okm
    }

    // ── Persistence ──────────────────────────────────────────────

    pub fn load_or_create() -> Result<Self, CoreError> {
        let path = Self::storage_path()?;
        if path.exists() {
            Self::load(&path)
        } else {
            let id = Self::generate();
            id.save(&path)?;
            tracing::info!(device_id = %id.id, "Created new device identity");
            Ok(id)
        }
    }

    fn generate() -> Self {
        let mut rng       = OsRng;
        let signing_key   = SigningKey::generate(&mut rng);
        let dh_secret     = StaticSecret::random_from_rng(&mut rng);
        let fingerprint   = hardware_fingerprint();
        let id            = Uuid::new_v5(&Uuid::NAMESPACE_DNS, fingerprint.as_bytes());
        let display_name  = hostname::get()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "Borderless Device".into());

        Self { id, fingerprint, display_name, signing_key, dh_secret }
    }

    fn storage_path() -> Result<PathBuf, CoreError> {
        let base = dirs::config_dir()
            .ok_or_else(|| CoreError::Identity("Cannot find config directory".into()))?;
        Ok(base.join("borderless").join("identity.json"))
    }

    fn save(&self, path: &PathBuf) -> Result<(), CoreError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let stored = StoredIdentity {
            id:           self.id,
            fingerprint:  self.fingerprint.clone(),
            display_name: self.display_name.clone(),
            signing_bytes: hex::encode(self.signing_key.to_bytes()),
            dh_bytes:     hex::encode(self.dh_secret.to_bytes()),
        };
        let json = serde_json::to_string_pretty(&stored)
            .map_err(CoreError::Serde)?;
        // Set restrictive permissions before writing
        std::fs::write(path, json)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }

    fn load(path: &PathBuf) -> Result<Self, CoreError> {
        let json  = std::fs::read_to_string(path)?;
        let s: StoredIdentity = serde_json::from_str(&json)
            .map_err(CoreError::Serde)?;

        let sign_bytes = hex::decode(&s.signing_bytes)
            .map_err(|e| CoreError::Identity(e.to_string()))?;
        let sign_arr: [u8; 32] = sign_bytes.try_into()
            .map_err(|_| CoreError::Identity("Bad signing key length".into()))?;

        let dh_bytes = hex::decode(&s.dh_bytes)
            .map_err(|e| CoreError::Identity(e.to_string()))?;
        let dh_arr: [u8; 32] = dh_bytes.try_into()
            .map_err(|_| CoreError::Identity("Bad DH key length".into()))?;

        Ok(Self {
            id:           s.id,
            fingerprint:  s.fingerprint,
            display_name: s.display_name,
            signing_key:  SigningKey::from_bytes(&sign_arr),
            dh_secret:    StaticSecret::from(dh_arr),
        })
    }
}

// ── Serialized form (stored on disk) ─────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
struct StoredIdentity {
    id:           Uuid,
    fingerprint:  String,
    display_name: String,
    signing_bytes: String, // hex
    dh_bytes:     String,  // hex
}

// ── Hardware fingerprint ──────────────────────────────────────────

fn hardware_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();

    // Hostname (stable on most setups)
    if let Ok(name) = hostname::get() {
        h.update(name.to_string_lossy().as_bytes());
    }

    // OS-specific machine ID
    #[cfg(target_os = "linux")]
    {
        // /etc/machine-id is stable across reboots, changes on re-install
        if let Ok(mid) = std::fs::read_to_string("/etc/machine-id") {
            h.update(mid.trim().as_bytes());
        } else if let Ok(mid) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
            h.update(mid.trim().as_bytes());
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
        {
            h.update(&out.stdout);
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Read HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid via reg query
        if let Ok(out) = std::process::Command::new("reg")
            .args(["query", "HKLM\\SOFTWARE\\Microsoft\\Cryptography", "/v", "MachineGuid"])
            .output()
        {
            h.update(&out.stdout);
        }
    }

    format!("{:x}", h.finalize())
}

// ── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_sign() {
        let id = DeviceIdentity::generate();
        let msg = b"hello borderless";
        let sig = id.sign(msg);
        assert!(id.verifying_key().verify_strict(msg, &sig).is_ok());
    }

    #[test]
    fn ecdh_both_sides_agree() {
        let alice = DeviceIdentity::generate();
        let bob   = DeviceIdentity::generate();
        let k_ab  = alice.derive_session_key(&bob.dh_public());
        let k_ba  = bob.derive_session_key(&alice.dh_public());
        assert_eq!(k_ab, k_ba);
    }

    #[test]
    fn uuid_is_deterministic_from_fingerprint() {
        let a = DeviceIdentity::generate();
        // Same fingerprint → same UUID
        let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, a.fingerprint.as_bytes());
        assert_eq!(a.id, id);
    }
}
