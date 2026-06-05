//! File transfer engine.
//!
//! Files are split into fixed-size chunks, each chunk is encrypted
//! with the session key, and transferred over the data channel.
//! On the receiver side chunks are reassembled and the final SHA-256
//! is verified before writing to disk.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::CoreError;

const CHUNK_SIZE: u32 = 64 * 1024; // 64 KiB per chunk

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Transferring { chunks_done: u32, total: u32 },
    Verifying,
    Complete,
    Failed(String),
    Cancelled,
}

/// Outbound transfer — reads a local file and emits chunks.
pub struct OutboundTransfer {
    pub id:           Uuid,
    pub file_path:    PathBuf,
    pub file_name:    String,
    pub file_size:    u64,
    pub sha256_hex:   String,
    pub total_chunks: u32,
    pub chunk_size:   u32,
}

impl OutboundTransfer {
    pub async fn prepare(path: &Path) -> Result<Self, CoreError> {
        use sha2::{Digest, Sha256};
        let metadata = tokio::fs::metadata(path).await?;
        let file_size = metadata.len();
        let total_chunks = ((file_size + CHUNK_SIZE as u64 - 1) / CHUNK_SIZE as u64) as u32;

        // Hash the whole file
        let mut file = tokio::fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; CHUNK_SIZE as usize];
        loop {
            let n = file.read(&mut buf).await?;
            if n == 0 { break; }
            hasher.update(&buf[..n]);
        }
        let sha256_hex = format!("{:x}", hasher.finalize());

        let file_name = path
            .file_name()
            .ok_or_else(|| CoreError::Transfer("No filename".into()))?
            .to_string_lossy()
            .into_owned();

        Ok(Self {
            id: Uuid::new_v4(),
            file_path: path.to_path_buf(),
            file_name,
            file_size,
            sha256_hex,
            total_chunks,
            chunk_size: CHUNK_SIZE,
        })
    }

    /// Read chunk `index` from the file.
    pub async fn read_chunk(&self, index: u32) -> Result<Vec<u8>, CoreError> {
        let offset = index as u64 * self.chunk_size as u64;
        let mut file = tokio::fs::File::open(&self.file_path).await?;
        use tokio::io::AsyncSeekExt;
        file.seek(std::io::SeekFrom::Start(offset)).await?;
        let mut buf = vec![0u8; self.chunk_size as usize];
        let n = file.read(&mut buf).await?;
        buf.truncate(n);
        Ok(buf)
    }
}

/// Inbound transfer — receives chunks and reassembles the file.
pub struct InboundTransfer {
    pub id:           Uuid,
    pub file_name:    String,
    pub file_size:    u64,
    pub sha256_hex:   String,
    pub total_chunks: u32,
    pub chunk_size:   u32,
    chunks:           HashMap<u32, Vec<u8>>,
}

impl InboundTransfer {
    pub fn new(
        id: Uuid, file_name: String, file_size: u64,
        sha256_hex: String, total_chunks: u32, chunk_size: u32,
    ) -> Self {
        Self { id, file_name, file_size, sha256_hex, total_chunks, chunk_size, chunks: HashMap::new() }
    }

    pub fn receive_chunk(&mut self, index: u32, data: Vec<u8>) {
        self.chunks.insert(index, data);
    }

    pub fn is_complete(&self) -> bool {
        self.chunks.len() as u32 == self.total_chunks
    }

    pub fn progress(&self) -> (u32, u32) {
        (self.chunks.len() as u32, self.total_chunks)
    }

    /// Reassemble chunks, verify SHA-256, and write to `dest_dir`.
    pub async fn finalise(&self, dest_dir: &Path) -> Result<PathBuf, CoreError> {
        use sha2::{Digest, Sha256};

        if !self.is_complete() {
            return Err(CoreError::Transfer("Not all chunks received".into()));
        }

        let dest = dest_dir.join(&self.file_name);
        let mut file   = tokio::fs::File::create(&dest).await?;
        let mut hasher = Sha256::new();

        for i in 0..self.total_chunks {
            let chunk = self.chunks
                .get(&i)
                .ok_or_else(|| CoreError::Transfer(format!("Missing chunk {i}")))?;
            hasher.update(chunk);
            file.write_all(chunk).await?;
        }
        file.flush().await?;

        let actual_hash = format!("{:x}", hasher.finalize());
        if actual_hash != self.sha256_hex {
            tokio::fs::remove_file(&dest).await.ok();
            return Err(CoreError::Transfer(
                format!("SHA-256 mismatch — file corrupt\n  expected: {}\n  got: {}",
                    self.sha256_hex, actual_hash)
            ));
        }

        tracing::info!(file = %dest.display(), "File transfer complete — verified OK");
        Ok(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn transfer_roundtrip() {
        let dir  = tempdir().unwrap();
        let src  = dir.path().join("test.txt");
        let mut f: tokio::fs::File = tokio::fs::File::create(&src).await.unwrap();
        // Write 130 KiB (3 chunks of 64 KiB)
        let data = vec![0x42u8; 130 * 1024];
        f.write_all(&data).await.unwrap();
        f.flush().await.unwrap();
        drop(f);

        let out  = OutboundTransfer::prepare(&src).await.unwrap();
        assert_eq!(out.total_chunks, 3);

        let mut inb = InboundTransfer::new(
            out.id, out.file_name.clone(), out.file_size,
            out.sha256_hex.clone(), out.total_chunks, out.chunk_size,
        );

        for i in 0..out.total_chunks {
            let chunk = out.read_chunk(i).await.unwrap();
            inb.receive_chunk(i, chunk);
        }

        assert!(inb.is_complete());
        let dest = inb.finalise(dir.path()).await.unwrap();
        let written = tokio::fs::read(&dest).await.unwrap();
        assert_eq!(written, data);
    }
}
