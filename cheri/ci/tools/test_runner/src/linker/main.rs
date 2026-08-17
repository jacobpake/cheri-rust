use std::path::PathBuf;

mod config;
mod linker;

use config::*;

struct Args {
    output_path: String,
    objects: Vec<String>,
    // TODO: unfortunately, we can't just exclude the libs we do not want,
    // as the ldscripts generated during `xmake config` expect them
    _needs_softfloat: bool,
    _needs_math: bool,
}

impl Args {
    fn from_env() -> Self {
        let argv: Vec<String> = std::env::args().collect();

        let mut output: Option<String> = None;
        let mut objects: Vec<String> = Vec::new();
        let mut needs_softfloat = false;
        let mut needs_math = false;

        let mut expect_output = false;

        for arg in argv {
            if expect_output {
                output = Some(arg);
                expect_output = false;
            } else if arg == "-o" {
                expect_output = true;
            } else if arg == "--needs-softfloat" {
                needs_softfloat = true;
            } else if arg == "--neds-math" {
                needs_math = true;
            } else if arg.ends_with(".o") || arg.ends_with(".rlib") {
                objects.push(arg);
            }
        }

        let output_path = output.unwrap();

        Self { output_path, objects, _needs_math: needs_math, _needs_softfloat: needs_softfloat }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_env();
    let root_dir = PathBuf::from(std::env::var("RUSTC_ROOT_DIR")?);

    let linker = linker::Linker::new(&root_dir);

    // TODO: unfortunately, we can't just exclude the libs we do not want,
    // as the ldscripts generated during `xmake config` expect them
    let libs = [CORE_LIBS, MATH_LIBS, SOFTFLOAT_LIBS].concat();

    linker.link_compartment(args.objects)?;
    linker.link_firmware(args.output_path, libs)?;

    Ok(())
}
