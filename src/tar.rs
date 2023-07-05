use crate::{ext, CommandExt};
use std::{ffi::OsStr, process::Command};

ext!(def; Tar);

impl Tar {
    /// 调用 `tar xf` 解压归档文件。
    #[inline]
    pub fn xf(src: impl AsRef<OsStr>, dst: Option<impl AsRef<OsStr>>) -> Self {
        let mut tar = Self(Command::new("tar"));
        tar.arg("xf").arg(src).optional(&dst, |tar, dst| {
            tar.arg("-C").arg(dst);
        });
        tar
    }
}
