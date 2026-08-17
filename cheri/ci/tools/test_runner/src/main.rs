//! # test-runner
//! This tool is designed to simplify running various `rustc` test suites
//! on our CHERIoT target, as a replacement for bootstrap (`x.py`).
//!
//! This tool currently supports:
//!     * `coretests`
//!
//! We require such a tool for various reasons, including:
//!     * `libtest` expects `std` features we do not or cannot support.
//!     * Suites such as `coretests` are compiled to a single binary which
//!       is too large for our platform.
//!     * Suites such as `compiletest/ui` want to link thousands of individual
//!       executables which takes a very long time on our platform.
//!
//! This tool is designed to be used in CI. If you want to run it locally, be
//! aware of the following:
//!     * You should run this tool from the root directory of the `cheri-rust`
//!       repo. If you are elsewhere, you must provide a `--root-dir` argument.
//!     * You must have built a compiler and "sysroot" for our "facade" target,
//!       e.g. `x build compiler std --target riscv32cheriot-unknown-cheriotrtos.facade`.
//!     * This tool does not detect changes to the "sysroot" crates ("core", "std", etc.).
//!       You need to rebuild the compiler and sysroot with the command above.
//!     * Other changes, except changes to our libtest or the test suite itself, will not
//!       be detected. The "--clean" argument will remove `cargo` and `xmake` build artifacts.

use clap::Parser;
use rayon::prelude::*;

use std::path::PathBuf;

use test_runner::cargo;
use test_runner::known_issues;
use test_runner::results;
use test_runner::runner;
use test_runner::xmake;

#[derive(Parser)]
struct Args {
    /// E.g. "coretests", "alloctests"
    suite: String,

    /// Specify a single module. If omitted will run all modules. E.g. "iter_num", "fmt"
    module: Option<String>,

    /// Simulator to use for test execution
    #[arg(long, default_value = "cheriot_sim", value_name = "BIN")]
    simulator: PathBuf,

    /// The root of the rustc project. If not set, assumes current working directory
    #[arg(long, default_value = ".", value_name = "DIR")]
    root_dir: PathBuf,

    /// Path to a known issues file
    #[arg(long, value_name = "FILE")]
    known_issues: Option<PathBuf>,

    /// Clean build directories
    #[arg(long)]
    clean: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let root_dir = args.root_dir.canonicalize()?;

    let cargo = cargo::Cargo::new(&root_dir);
    let xmake = xmake::Xmake::new(&root_dir);

    if args.clean {
        // runs `cargo clean` on sysroot/alloc and cleans up xmake directories
        cargo.clean()?;
    }

    xmake.config(xmake::XmakeConfig { needs_softfloat: true, needs_math: true })?;
    xmake.build()?;

    let modules = match args.module {
        Some(module) => Vec::from([module]),
        // we extract all the `test_` features from the crate metadata
        None => cargo.get_features(&args.suite).unwrap(),
    };

    let known_issues = match args.known_issues {
        Some(path) => known_issues::KnownIssues::from_file(&path).unwrap(),
        None => known_issues::KnownIssues::default(),
    };

    let t0 = std::time::Instant::now();

    let results = modules
        .par_iter()
        .map(|module| {
            println!("Build {}/{} ... started", args.suite, module);
            let executable = cargo.build_test_executable(&args.suite, module)?;
            println!("Build {}/{} ... ok", args.suite, module);
            println!("Run {}/{} ... started", args.suite, module);
            let runner = runner::Runner::new(&args.simulator, &executable, &known_issues);
            let results = runner.run()?;

            Ok(results)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .fold(results::Results::default(), |acc, r| acc + r);

    let t1 = std::time::Instant::now();

    println!("time: {}ms", (t1 - t0).as_millis(),);

    let failures = results.get_failures();

    if !failures.is_empty() {
        println!(
            "\nFailing:\n{}",
            failures
                .iter()
                .map(|(test, failure_mode)| format!(
                    "  {}{}",
                    test,
                    match failure_mode {
                        results::FailureMode::UnexpectedFail => " - failed",
                        results::FailureMode::UnexpectedPass => " - ok, but in known issues",
                        results::FailureMode::UnexpectedIgnore => " - ignored, but in known issues",
                    }
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
        anyhow::bail!("test suite unsuccessful")
    }

    Ok(())
}
