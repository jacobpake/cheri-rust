use std::{path::PathBuf, process::Command};

use crate::utils;

pub struct Xmake {
    llvm_sdk_dir: PathBuf,
    project_dir: PathBuf,
}

pub struct XmakeConfig {
    pub needs_math: bool,
    pub needs_softfloat: bool,
}

impl Xmake {
    pub fn new(root_dir: &PathBuf) -> Self {
        Self {
            llvm_sdk_dir: utils::get_llvm_sdk_dir(root_dir),
            project_dir: root_dir.join("cheri/ci/build"),
        }
    }

    pub fn config(&self, config: XmakeConfig) -> anyhow::Result<()> {
        let cmd = Command::new("xmake")
            .arg("config")
            .args([
                format!("--sdk={}", self.llvm_sdk_dir.to_str().unwrap()).as_str(),
                format!("--needs-math={}", if config.needs_math { "y" } else { "n" }).as_str(),
                format!("--needs-softfloat={}", if config.needs_softfloat { "y" } else { "n" })
                    .as_str(),
            ])
            .current_dir(&self.project_dir)
            .output()?;

        utils::cmd_expect_success(cmd)
    }

    pub fn build(&self) -> anyhow::Result<()> {
        let cmd = Command::new("xmake").arg("build").current_dir(&self.project_dir).output()?;

        utils::cmd_expect_success(cmd)
    }
}
