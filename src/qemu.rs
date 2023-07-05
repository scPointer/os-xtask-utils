use super::ext;
use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
};

ext!(def; Qemu);

// Waiting for `std::cell::LazyCell`.
static SEARCH_DIRS: once_cell::sync::Lazy<Mutex<HashSet<PathBuf>>> =
    once_cell::sync::Lazy::new(|| {
        Mutex::new(if cfg!(target_os = "windows") {
            HashSet::from_iter([PathBuf::from(r"C:\Program Files\qemu")])
        } else {
            HashSet::new()
        })
    });

impl Qemu {
    /// Qemu 添加搜索路径。
    #[inline]
    pub fn search_at(path: impl AsRef<Path>) {
        SEARCH_DIRS
            .lock()
            .unwrap()
            .insert(path.as_ref().to_path_buf());
    }

    #[inline]
    fn find(name: impl AsRef<OsStr>) -> Self {
        Self(Command::new(Self::find_qemu(OsString::from_iter([
            OsStr::new("qemu-"),
            name.as_ref(),
        ]))))
    }

    /// 调用 Qemu system 虚拟化。
    #[inline]
    pub fn system(arch: impl AsRef<OsStr>) -> Self {
        Self::find(OsString::from_iter([OsStr::new("system-"), arch.as_ref()]))
    }

    /// 调用 Qemu img。
    #[inline]
    pub fn img() -> Self {
        Self::find("img")
    }

    fn find_qemu(#[allow(unused_mut)] mut name: OsString) -> OsString {
        #[cfg(target_os = "windows")]
        name.push(OsStr::new(".exe"));
        SEARCH_DIRS
            .lock()
            .unwrap()
            .iter()
            .map(|dir| dir.join(&name))
            .find(|path| path.is_file())
            .map_or(name, |p| p.into_os_string())
    }
}
