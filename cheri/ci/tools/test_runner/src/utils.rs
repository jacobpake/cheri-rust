use std::{path::PathBuf, process::Output};

pub fn get_llvm_sdk_dir(root_dir: &PathBuf) -> PathBuf {
    if let Ok(path) = std::env::var("CHERIOT_SYSROOT_DIR") {
        return PathBuf::from(path);
    }
    return root_dir.join("build/host/llvm");
}

pub fn cmd_expect_success(output: Output) -> anyhow::Result<()> {
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        eprintln!("{}", stderr);
        anyhow::bail!("");
    }
    Ok(())
}
