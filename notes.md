# Running doctests

We want to run `x test ...` to take advantage of the Rust test infrastructure and get some wide coverage. This builds and runs an executable for each test, and depends on having `std` compile for our target.

As well as these changes, you also need the following:

In `bootstrap.toml`:

```toml
[target.riscv32cheriot-unknown-cheriotrtos]
linker="./cheri/linker.sh"
runner="cheriot_sim" # or path to sail sim
```

There is an invalid use of `transmute` in `hashbrown` [here](https://github.com/rust-lang/hashbrown/blob/bba4a01bc6a29d11b8aef16b4ebeec710e3f6e89/src/util.rs#L37) which needs to be patched. For now I just changed to `without_provenance_mut` in `~/.cargo/registry/src/...` for `hashbrown-0.15.5`.

## `AtomicUsize`

Currently we do not build `AtomicUsize` which std depends on. It seems to be guarded behind config flags that expect our target `max_atomic_width` to equal our `pointer_width`. I suspect we will be able to patch this to compare with `usize`. To compile in the meantime we can set `max_atomic_width` to 64. I am not expecting atomics to be used in the tests but also set `singlethread: true`.

Relevant:

[library/core/src/sync/atomic.rs](https://github.com/CHERIoT-Platform/cheri-rust/blob/beta/library/core/src/sync/atomic.rs)
[compiler/rustc_session/src/config/cfg.rs](https://github.com/CHERIoT-Platform/cheri-rust/blob/beta/compiler/rustc_session/src/config/cfg.rs)

## Standard library

Targets can provide implementations for the APIs in standard library. Many of these have an `unsupported` implementation which is used by default or behind configuration flags. The absolute minimum required is an `alloc` implementation. Edoardo suggests we can ffi to malloc/free here as in the test runner. Some other platforms to do similar.

We can incrementally add support here as we want. There are other targets doing similar, `wasm`, `zkvm`, probably others are useful examples.

## Other issues

There are some errors during instruction selection which were avoided by disabling fp16 [here](https://github.com/jacobpake/cheri-rust/blob/37d11537aecb490f57651e545b6b52a4b5851dce/compiler/rustc_codegen_llvm/src/llvm_util.rs#L339-L363) and removing the use of an operator [here](https://github.com/jacobpake/cheri-rust/blob/37d11537aecb490f57651e545b6b52a4b5851dce/library/test/src/bench.rs#L106).

## Running

Make changes to `bootstrap.toml` specified above.

Build with `x build compiler std --target=riscv32cheriot-unknown-cheriotrtos`.

In `cheri/doctests`, run `xmake config --sdk=$LLVM_PATH` where `LLVM_PATH` is

To setup, run `x config --sdk=$LLVM_PATH` in `cheri/doctests` where `LLVM_PATH` is your
CHERIoT LLVM build directory (e.g. `../../build/host/llvm`).

Run the tests with:

```sh
x test library/core --target riscv32cheriot-unknown-cheriotrtos --doc --jobs 1  -- --no-capture
```

- `--doc` to only run doctests (libtest tests not yet supported),
- `--jobs 1` because we are reusing the xmake build directory, this turns out more performant
- `-- --no-capture` so we can get useful information about compile/runtime/link errors

To run a subset of tests you can add filters to the end, as either file or function names e.g.
`... -- --no-capture library/core/src/primitive_docs.rs prim_char ...`.

I have added the latest output from running the full suite in `test-output.txt`.
