use std::collections::HashSet;
use self::cache::CacheManager;
use md5::digest::FixedOutput;
use md5::{Digest, Md5};
use std::fs::{read_dir, DirEntry, File};
use std::{io, fs, iter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

mod cache;

pub enum SaveKind {
    Save,
    CurrentBackup,
    Backup,
}

pub struct Save {
    pub kind: SaveKind,
    pub name: String,
    pub created_at: SystemTime,
}

pub fn async_list_saves<F>(f: F)
where
    F: FnOnce(Vec<Save>) + Send + 'static,
{
    std::thread::spawn(|| {
        let saves = list_saves();
        f(saves)
    });
}

pub fn async_list_backups<F>(name: String, f: F)
where
    F: FnOnce(Vec<Save>) + Send + 'static,
{
    std::thread::spawn(move || {
        let saves = list_backups(&name).into_saves();
        f(saves)
    });
}

pub fn async_backup<F>(name: String, f: F)
where F: FnOnce(Vec<Save>) + Send + 'static {
    std::thread::spawn(move || {
        let backups = list_backups(&name);
        let _ = backup(&backups);
        let saves = list_backups(&name).into_saves();
        f(saves)
    });
}

pub fn async_restore<F>(name: String, backup: String, f: F)
    where F: FnOnce(Vec<Save>) + Send + 'static {
    std::thread::spawn(move || {
        let backups = list_backups(&name);
        let _ = restore(&backups, &backup);
        let saves = list_backups(&name).into_saves();
        f(saves)
    });
}

const SAVE_EXT: &str = "ck3";

pub(crate) struct SaveFile {
    path: PathBuf,
    name: String,
    ext: String,
    created_at: SystemTime,
    hash: Option<Vec<u8>>,
}

impl SaveFile {
    fn hash_file(path: &Path) -> Option<Vec<u8>> {
        println!("Begin hasing {}", path.display());
        let mut file = File::open(path).ok()?;
        let mut hash = Md5::new();
        let _ = io::copy(&mut file, &mut hash).ok()?;
        let result = hash.finalize_fixed().to_vec();
        println!("Done hasing {}", path.display());
        Some(result)
    }

    fn from_entry(entry: DirEntry) -> Option<Self> {
        let file_type = entry.file_type().ok()?;
        if file_type.is_file() {
            let path = entry.path();

            let name = path.file_name()?;
            let name = name.to_string_lossy().to_string();

            let ext = path.extension()?;
            let ext = ext.to_string_lossy().to_string();

            let metadata = entry.metadata().ok()?;
            let created = metadata.created().ok()?;

            Some(SaveFile {
                path,
                name,
                ext,
                created_at: created,
                hash: None,
            })
        } else {
            None
        }
    }

    fn with_hash(self) -> Self {
        let mut save = CacheManager::with_hash(self);
        if save.hash.is_none() {
            save.hash = Self::hash_file(&save.path);
            let _ = CacheManager::save(&save);
        }
        save
    }

    fn into_save(self) -> Save {
        self.into_with_kind(SaveKind::Save)
    }

    fn into_current_backup(self) -> Save {
        self.into_with_kind(SaveKind::CurrentBackup)
    }

    fn into_backup(self) -> Save {
        self.into_with_kind(SaveKind::Backup)
    }

    fn into_with_kind(self, kind: SaveKind) -> Save {
        Save {
            kind,
            name: self.name,
            created_at: self.created_at,
        }
    }
}

fn save_dir() -> Option<PathBuf> {
    let data_dir = dirs::data_dir()?;
    let save_dir = data_dir
        .join("Paradox Interactive")
        .join("Crusader Kings III")
        .join("save games");
    Some(save_dir)
}

fn list_save_iter() -> Option<impl Iterator<Item = SaveFile>> {
    let save_dir = save_dir()?;
    let entries = read_dir(save_dir).ok()?;
    let files = entries
        .filter_map(Result::ok)
        .filter_map(SaveFile::from_entry);
    Some(files)
}

fn sort_save_files(mut files: Vec<SaveFile>) -> Vec<SaveFile> {
    files.sort_by(|first, second| first.created_at.cmp(&second.created_at).reverse());
    files
}

fn list_saves() -> Vec<Save> {
    if let Some(iter) = list_save_iter() {
        let files = iter.filter(|file| file.ext == SAVE_EXT).collect::<Vec<_>>();
        let files = sort_save_files(files);
        files.into_iter().map(SaveFile::into_save).collect()
    } else {
        Vec::new()
    }
}

struct Backups<'a> {
    name: &'a str,
    hash: Option<Vec<u8>>,
    current_save: Option<SaveFile>,
    current_backup: Option<SaveFile>,
    backups: Vec<SaveFile>,
}

impl<'a> Backups<'a> {
    fn new(name: &'a str, hash: Option<Vec<u8>>) -> Self {
        Backups {
            name,
            hash,
            current_save: None,
            current_backup: None,
            backups: Vec::new(),
        }
    }

    fn fold(&mut self, save: SaveFile) -> &mut Self {
        if self.hash.is_some() {
            if save.hash == self.hash {
                if save.ext == SAVE_EXT {
                    self.current_save = Some(save);
                } else {
                    self.current_backup = Some(save)
                }
            } else {
                self.backups.push(save);
            }
        } else {
            self.backups.push(save);
        }
        self
    }

    fn into_saves(self) -> Vec<Save> {
        if let Some(current_save) = self.current_save {
            if let Some(current_backup) = self.current_backup {
                let current_backup = current_backup.into_current_backup();
                let backups = self.backups.into_iter().map(SaveFile::into_backup);
                iter::once(current_backup).chain(backups).collect()
            } else {
                vec![current_save.into_save()]
            }
        } else {
            Vec::new()
        }
    }
}

fn list_backups(name: &str) -> Backups {
    if let Some(iter) = list_save_iter() {
        let files = iter
            .filter(|file| file.name.starts_with(&name))
            .map(SaveFile::with_hash)
            .collect::<Vec<_>>();
        let files = sort_save_files(files);

        let hash = files
            .iter()
            .filter(|save| save.ext == SAVE_EXT)
            .flat_map(|save| save.hash.clone())
            .next();

        let mut backups = Backups::new(name, hash);
        files.into_iter().fold(&mut backups, Backups::fold);
        backups
    } else {
        Backups::new(name, None)
    }
}

fn find_backup_name(backups: &Backups) -> Option<String> {
    // Check that there is a save
    backups.current_save.as_ref()?;

    let iter = backups.current_backup.iter().chain(backups.backups.iter());
    let exts = iter.map(|save| save.ext.clone()).collect::<HashSet<_>>();

    let ext = iter::successors(Some(1), |i| Some(i + 1))
        .map(|i| format!("backup{:02}", i))
        .filter(|ext| !exts.contains(ext))
        .next()?;
    Some(format!("{}.{}", backups.name, ext))
}

fn backup(backups: &Backups) -> Option<()> {
    let save_dir = save_dir()?;
    let target_name = find_backup_name(backups)?;

    let source = save_dir.join(&backups.name);
    let target = save_dir.join(target_name);
    fs::copy(source, target).ok()?;
    Some(())
}

fn check_restore(backups: &Backups, backup: &str) -> Option<String> {
    // Check that there is a save and a backup for current save
    let save = backups.current_save.as_ref()?;
    backups.current_backup.as_ref()?;

    // Check that current backup is valid
    backups.backups.iter().find(|save| save.name == backup)?;
    Some(save.name.clone())
}

fn restore(backups: &Backups, backup: &str) -> Option<()> {
    let save_dir = save_dir()?;
    let target_name = check_restore(backups, backup)?;

    let source = save_dir.join(backup);
    let target = save_dir.join(target_name);
    fs::remove_file(&target).ok()?;
    fs::copy(source, target).ok()?;
    Some(())
}