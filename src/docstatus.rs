use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug, info};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MatrixFile {
    pub(crate) file: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Matrix {
    pub(crate) include: Vec<MatrixFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Badge {
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileStatus {
    status: bool,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DocStatus {
    badge: Badge,
    files: HashMap<String, FileStatus>,
    interval: u64,
}

#[derive(Debug, Clone)]
pub struct DocStatusError {}

impl DocStatus {
    //TODO: list untraced files

    pub(crate) fn list(&self, all: &bool) -> anyhow::Result<()> {
        info!("list {all}");
        Ok(())
    }

    pub(crate) fn fix(&self, article: &str) -> anyhow::Result<()> {
        info!("fix {}", article);
        Ok(())
    }

    pub(crate) fn ensure_badge(&self, article: &Option<String>) -> anyhow::Result<()> {
        info!("ensure-badge {:?}", article);
        Ok(())
    }

    pub(crate) fn check(&mut self) -> anyhow::Result<Vec<String>> {
        let mut changed_files = vec![];
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let interval = self.interval;
        debug!(
            "Check at {} for interval {} outdated from {}",
            timestamp,
            interval,
            timestamp - interval
        );
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name().to_string_lossy().ends_with(".md") {
                let f_path = entry.path().to_string_lossy();
                let f_path = f_path.split_at(2).1; // skip preceding ./
                if let Some(file_status) = self.files.get_mut(f_path) {
                    debug!(
                        "Checking {} with timestamp {}",
                        &f_path, &file_status.timestamp
                    );
                    if file_status.status && file_status.timestamp + self.interval < timestamp {
                        info!("Marking {} as status = false", &f_path);
                        changed_files.push(f_path.to_string());
                        file_status.status = false;
                    }
                }
            }
        }
        Ok(changed_files)
    }
}

impl From<&Path> for DocStatus {
    fn from(config: &Path) -> Self {
        let file = File::open(config).unwrap();
        let reader = BufReader::new(file);
        let doc_status: DocStatus =
            serde_json::from_reader(reader).expect("Must point to valid config");
        doc_status
    }
}
