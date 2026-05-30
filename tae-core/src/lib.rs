use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::{Context, Result, bail};
use zip::ZipArchive;

// 32-byte hardcoded key (Option B). Replace all bytes to change the key.
const PACK_KEY: [u8; 32] = [
    0x74, 0x61, 0x65, 0x5f, 0x6d, 0x76, 0x70, 0x5f,
    0x6b, 0x65, 0x79, 0x5f, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31,
];

// Nonce is fixed for MVP (12 bytes). Rotate per-file post-MVP.
const NONCE: [u8; 12] = [0x74, 0x61, 0x65, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x30, 0x30, 0x30, 0x31];

/// In-memory representation of all loaded game files: path → raw bytes.
pub type FileStore = HashMap<String, Vec<u8>>;

/// Load from `data/` folder if present, otherwise from `data.tae`.
/// Checks `base` first (next to executable), then current working directory.
pub fn load(base: &Path) -> Result<FileStore> {
    let candidates = [
        base.to_path_buf(),
        std::env::current_dir().unwrap_or_else(|_| base.to_path_buf()),
    ];
    for dir in &candidates {
        let folder = dir.join("data");
        if folder.is_dir() {
            return load_folder(&folder);
        }
        let archive = dir.join("data.tae");
        if archive.is_file() {
            return load_archive(&archive);
        }
    }
    bail!("No data/ folder or data.tae file found (checked next to executable and working directory)")
}

pub fn load_folder(dir: &Path) -> Result<FileStore> {
    let mut store = FileStore::new();
    visit_dir(dir, dir, &mut store)?;
    Ok(store)
}

fn visit_dir(root: &Path, dir: &Path, store: &mut FileStore) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dir(root, &path, store)?;
        } else {
            let rel = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
            let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
            store.insert(rel, bytes);
        }
    }
    Ok(())
}

fn load_archive(path: &Path) -> Result<FileStore> {
    let encrypted = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let zip_bytes = decrypt(&encrypted)?;
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut zip = ZipArchive::new(cursor).context("opening zip archive")?;
    let mut store = FileStore::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        if entry.is_file() {
            let name = entry.name().replace('\\', "/");
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;
            store.insert(name, bytes);
        }
    }
    Ok(store)
}

pub fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(&PACK_KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&NONCE);
    cipher.decrypt(nonce, data).map_err(|e| anyhow::anyhow!("decryption failed: {e}"))
}

pub fn encrypt(data: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(&PACK_KEY);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&NONCE);
    cipher.encrypt(nonce, data).map_err(|e| anyhow::anyhow!("encryption failed: {e}"))
}

pub fn get_text<'a>(store: &'a FileStore, path: &str) -> Result<&'a str> {
    let bytes = store.get(path).with_context(|| format!("missing file: {path}"))?;
    std::str::from_utf8(bytes).with_context(|| format!("invalid UTF-8 in {path}"))
}
