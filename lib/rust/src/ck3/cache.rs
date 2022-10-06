use super::SaveFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::SystemTime;

pub(crate) struct CacheManager;

#[derive(Debug, Deserialize, Serialize)]
struct CachedSave {
    timestamp: SystemTime,
    hash: Vec<u8>,
}

type Cache = HashMap<String, CachedSave>;

impl CacheManager {
    pub(crate) fn save(save: &SaveFile) -> Option<()> {
        let mut cache = Self::load_cache()?;

        let hash = save.hash.as_ref()?;
        cache.insert(
            save.name.clone(),
            CachedSave {
                timestamp: save.created_at,
                hash: hash.clone(),
            },
        );
        Self::save_cache(cache)
    }

    pub(crate) fn with_hash(mut save: SaveFile) -> SaveFile {
        if let Some(hash) = Self::load_hash(&save) {
            save.hash = Some(hash);
        }
        save
    }

    fn load_cache() -> Option<Cache> {
        let path = Self::cache_file()?;

        if path.exists() {
            let cache_file = File::open(path).ok()?;
            let reader = BufReader::new(cache_file);
            serde_json::from_reader(reader).ok()
        } else {
            Some(HashMap::new())
        }
    }

    fn save_cache(cache: Cache) -> Option<()> {
        let path = Self::cache_file()?;
        let cache_file = File::create(path).ok()?;
        serde_json::to_writer(cache_file, &cache).ok()?;
        Some(())
    }

    fn cache_file() -> Option<PathBuf> {
        let cache_dir = dirs::cache_dir()?.join("SKDeck");
        fs::create_dir_all(&cache_dir).ok()?;
        Some(cache_dir.join("cache.json"))
    }

    fn load_hash(save: &SaveFile) -> Option<Vec<u8>> {
        let cache = Self::load_cache()?;
        if let Some(cached) = cache.get(&save.name) {
            if save.created_at == cached.timestamp {
                Some(cached.hash.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}
