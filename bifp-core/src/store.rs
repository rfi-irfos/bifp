//! Real, native-trit persistence for BIFP flags — a JSONL-backed store where `trit` is a
//! first-class signed column, not folded into a tags array.
//!
//! This does NOT modify or extend RFI-IRFOS's hosted `ternlang-engram` MCP server (its
//! source isn't available on this machine, and it is a separately published, versioned
//! package). It is a new, independent store, purpose-built for BIFP, that closes the same
//! *gap* honestly instead of claiming to patch code this crate has no access to.

use crate::flag::Flag;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct FlagStore {
    path: PathBuf,
}

impl FlagStore {
    pub fn open(path: impl AsRef<Path>) -> Self {
        FlagStore { path: path.as_ref().to_path_buf() }
    }

    /// Append one flag as a JSON line. Never rewrites prior entries — append-only, so a
    /// revision (`Flag::revises`) is a new line pointing back, not an edit in place.
    pub fn remember(&self, flag: &Flag) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path)?;
        let line = serde_json::to_string(flag).map_err(io::Error::other)?;
        writeln!(f, "{line}")
    }

    /// All flags, in insertion order.
    pub fn recall_all(&self) -> io::Result<Vec<Flag>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let f = File::open(&self.path)?;
        let mut out = Vec::new();
        for line in BufReader::new(f).lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(flag) = serde_json::from_str::<Flag>(&line) {
                out.push(flag);
            }
        }
        Ok(out)
    }

    /// How many affirms (trit=+1) have been recorded for a given note-prefix — the crate's
    /// own reference implementation of "reconfirmed across multiple recalls" for badge::BadgeContext.
    pub fn reconfirm_count(&self, note_prefix: &str) -> io::Result<u32> {
        Ok(self
            .recall_all()?
            .iter()
            .filter(|f| f.trit == 1 && f.note.starts_with(note_prefix))
            .count() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::Source;
    use crate::trit::Trit;

    #[test]
    fn remember_creates_missing_parent_directory() {
        // Regression test: the real MCP server hit this live on 2026-07-02 — the
        // default path's parent dir (~/.bifp/) didn't exist yet, so `remember` silently
        // returned an Err and the tool reported `stored: false`.
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("does").join("not").join("exist").join("flags.jsonl");
        let store = FlagStore::open(&nested);
        let f = Flag::new(Trit::Affirm, 0.9, "regress", Source::Agent, "t");
        assert!(store.remember(&f).is_ok());
        assert_eq!(store.recall_all().unwrap().len(), 1);
    }

    #[test]
    fn remember_and_recall_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let store = FlagStore::open(dir.path().join("flags.jsonl"));
        let f1 = Flag::new(Trit::Affirm, 0.9, "test:one", Source::Agent, "2026-07-02T20:00:00Z");
        store.remember(&f1).unwrap();
        let all = store.recall_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].trit, 1);
    }

    #[test]
    fn reconfirm_count_counts_affirms_only() {
        let dir = tempfile::tempdir().unwrap();
        let store = FlagStore::open(dir.path().join("flags.jsonl"));
        store.remember(&Flag::new(Trit::Affirm, 0.9, "x:1", Source::Agent, "t1")).unwrap();
        store.remember(&Flag::new(Trit::Affirm, 0.8, "x:2", Source::Human, "t2")).unwrap();
        store.remember(&Flag::new(Trit::Reject, 0.5, "x:3", Source::Agent, "t3")).unwrap();
        assert_eq!(store.reconfirm_count("x:").unwrap(), 2);
    }
}
