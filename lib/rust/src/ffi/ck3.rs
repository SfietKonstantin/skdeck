use crate::ck3::{async_backup, async_list_backups, async_list_saves, async_restore, Save, SaveKind};
use crate::ffi::CtxWrapper;
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;
use std::time::SystemTime;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum RsCk3SaveKind {
    Save,
    CurrentBackup,
    Backup,
}

pub struct RsCk3Save {
    kind: RsCk3SaveKind,
    name: CString,
    created_at: i64,
}

impl From<SaveKind> for RsCk3SaveKind {
    fn from(kind: SaveKind) -> Self {
        match kind {
            SaveKind::Save => Self::Save,
            SaveKind::CurrentBackup => Self::CurrentBackup,
            SaveKind::Backup => Self::Backup,
        }
    }
}

impl RsCk3Save {
    fn try_from(save: Save) -> Option<RsCk3Save> {
        let name = CString::new(save.name.as_bytes()).ok()?;
        let created_at = match save.created_at.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_millis() as i64,
            Err(e) => -(e.duration().as_millis() as i64),
        };
        Some(RsCk3Save {
            kind: RsCk3SaveKind::from(save.kind),
            name,
            created_at,
        })
    }
}

#[no_mangle]
pub extern "C" fn ck3_save_kind(save: &RsCk3Save) -> RsCk3SaveKind {
    save.kind
}

#[no_mangle]
pub extern "C" fn ck3_save_name(save: &RsCk3Save) -> *const c_char {
    save.name.as_ptr()
}

#[no_mangle]
pub extern "C" fn ck3_save_created_at(save: &RsCk3Save) -> i64 {
    save.created_at
}

pub struct RsCk3Saves {
    saves: Vec<RsCk3Save>,
}

impl RsCk3Saves {
    fn from(saves: Vec<Save>) -> RsCk3Saves {
        let saves = saves.into_iter().filter_map(RsCk3Save::try_from).collect();
        RsCk3Saves { saves }
    }
}

#[no_mangle]
pub extern "C" fn ck3_saves_size(saves: &RsCk3Saves) -> usize {
    saves.saves.len()
}

#[no_mangle]
pub extern "C" fn ck3_saves_at(saves: &RsCk3Saves, i: usize) -> *const RsCk3Save {
    if let Some(save) = saves.saves.get(i) {
        save
    } else {
        ptr::null()
    }
}

type Ck3LoadSavesCb = extern "C" fn(*mut c_void, *const RsCk3Saves);

#[no_mangle]
pub extern "C" fn ck3_list_saves(ctx: *mut c_void, callback: Ck3LoadSavesCb) {
    let w = CtxWrapper::new(ctx);
    async_list_saves(move |saves| {
        let w = w;
        let saves = RsCk3Saves::from(saves);
        callback(w.ctx, &saves);
    })
}

#[no_mangle]
pub unsafe extern "C" fn ck3_list_backups(
    ctx: *mut c_void,
    name: *const c_char,
    callback: Ck3LoadSavesCb,
) {
    let w = CtxWrapper::new(ctx);
    let name = CStr::from_ptr(name);
    let name = name.to_str();

    if let Ok(name) = name {
        let name = name.to_string();
        async_list_backups(name, move |saves| {
            let w = w;
            let saves = RsCk3Saves::from(saves);
            callback(w.ctx, &saves)
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn ck3_backup(
    ctx: *mut c_void,
    name: *const c_char,
    callback: Ck3LoadSavesCb,
) {
    let w = CtxWrapper::new(ctx);
    let name = CStr::from_ptr(name);
    let name = name.to_str();

    if let Ok(name) = name {
        let name = name.to_string();
        async_backup(name, move |saves| {
            let w = w;
            let saves = RsCk3Saves::from(saves);
            callback(w.ctx, &saves)
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn ck3_restore(
    ctx: *mut c_void,
    name: *const c_char,
    save: *const c_char,
    callback: Ck3LoadSavesCb,
) {
    let w = CtxWrapper::new(ctx);
    let name = CStr::from_ptr(name);
    let name = name.to_str();

    let backup = CStr::from_ptr(save);
    let backup = backup.to_str();

    if let (Ok(name), Ok(backup)) = (name, backup) {
        let name = name.to_string();
        let backup = backup.to_string();
        async_restore(name, backup, move |saves| {
            let w = w;
            let saves = RsCk3Saves::from(saves);
            callback(w.ctx, &saves)
        })
    }
}
