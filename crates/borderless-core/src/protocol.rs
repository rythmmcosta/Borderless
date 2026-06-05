//! Wire protocol — envelope format for all inter-device messages.

use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub sender:      Uuid,
    pub target:      Uuid,
    pub msg_type:    MsgType,
    pub payload:     Vec<u8>,
    pub seq:         u32,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MsgType {
    Handshake,
    Input,
    Clipboard,
    FileTransferInit,
    FileTransferChunk,
    FileTransferAck,
    Notification,
    ScreenFrame,
    Ping,
    Pong,
    CursorWarp,
    CursorReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakePayload {
    pub device_id:       Uuid,
    pub display_name:    String,
    pub platform:        String,
    pub version:         String,
    pub dh_pubkey_hex:   String,
    pub sign_pubkey_hex: String,
    pub signature_hex:   String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorWarp {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransferInit {
    pub transfer_id:  Uuid,
    pub file_name:    String,
    pub file_size:    u64,
    pub mime_type:    String,
    pub sha256_hex:   String,
    pub chunk_size:   u32,
    pub total_chunks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub transfer_id:  Uuid,
    pub index:        u32,
    pub data:         Vec<u8>,
    pub is_last:      bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunkAck {
    pub transfer_id: Uuid,
    pub index:       u32,
    pub ok:          bool,
}
