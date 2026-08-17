use std::{path::PathBuf, process::Command};

use crate::config::*;
use test_runner::utils;

pub struct Linker {
    lld_bin: PathBuf,

    xmake_project_dir: PathBuf,

    build_dir: PathBuf,
    build_prefix: PathBuf,
}

impl Linker {
    pub fn new(root_dir: &PathBuf) -> Self {
        let llvm_sdk_dir = utils::get_llvm_sdk_dir(root_dir);

        let build_dir = PathBuf::from("build");
        let build_prefix = PathBuf::from("cheriot/cheriot/release");

        Self {
            lld_bin: llvm_sdk_dir.join("bin/ld.lld"),
            xmake_project_dir: root_dir.join("cheri/ci/build"),

            build_dir,
            build_prefix,
        }
    }

    fn lld_cmd(&self) -> Command {
        let mut cmd = Command::new(&self.lld_bin);

        cmd.current_dir(&self.xmake_project_dir);

        cmd
    }

    fn bin_path(&self, name: &str, extension: &str) -> PathBuf {
        self.build_dir.join(&self.build_prefix).join(name).with_added_extension(extension)
    }

    fn obj_path(&self, compartment: &str, name: &str) -> PathBuf {
        self.build_dir
            .join(".objs")
            .join(compartment)
            .join(&self.build_prefix)
            .join(name)
            .with_added_extension("o")
    }

    fn compartment_bin_path(&self, name: &str) -> PathBuf {
        self.bin_path(name, "compartment")
    }

    fn library_bin_path(&self, name: &str) -> PathBuf {
        self.bin_path(name, "library")
    }

    fn library_bin_paths(&self, libraries: Vec<&str>) -> Vec<PathBuf> {
        libraries.iter().map(|library| self.library_bin_path(library)).collect()
    }

    pub fn link_compartment(&self, rust_objects: Vec<String>) -> anyhow::Result<()> {
        let ldscript = &self.xmake_project_dir.join(COMPARTMENT_LDSCRIPT);
        // e.g. build/cheriot/cheriot/release/test_runner.compartment
        let output = self.compartment_bin_path(TEST_RUNNER_COMPARTMENT);
        // e.g. build/.objs/test_runner/cheriot/cheriot/release/runner.cc.o
        let cpp_wrapper_object = self.obj_path(TEST_RUNNER_COMPARTMENT, TEST_RUNNER_CPP_WRAPPER);

        let cmd = self
            .lld_cmd()
            .args([
                format!("--script={}", ldscript.to_str().unwrap()).as_str(),
                "--compartment",
                "--gc-sections",
                "--relax",
                "-o",
                output.to_str().unwrap(),
            ])
            .arg(cpp_wrapper_object)
            .args(rust_objects)
            .output()?;

        utils::cmd_expect_success(cmd)
    }

    pub fn link_firmware(&self, output: String, libs: Vec<&str>) -> anyhow::Result<()> {
        let libs = self.library_bin_paths(libs);
        let mmio_ldscript = self.build_dir.join("mmio.ldscript");
        let firmware_ldscript =
            self.build_dir.join(format!("{}-firmware.ldscript", TEST_RUNNER_FIRMWARE));

        let cmd = self
            .lld_cmd()
            .args([
                "--threads=1",
                "-n",
                format!("--script={}", mmio_ldscript.to_str().unwrap()).as_str(),
                format!("--script={}", firmware_ldscript.to_str().unwrap()).as_str(),
                "--relax",
                "-o",
            ])
            .arg(output)
            .args([
                self.obj_path("cheriot.switcher", "cheriot-rtos/sdk/core/switcher/entry.S"),
                self.bin_path("cheriot.loader.loader", "o"),
                self.compartment_bin_path("cheriot.allocator"),
                self.compartment_bin_path("cheriot.software_revoker"),
                self.compartment_bin_path("test_fw.scheduler"),
                self.compartment_bin_path("test_runner"),
            ])
            .args(libs)
            .output()?;

        utils::cmd_expect_success(cmd)
    }
}
