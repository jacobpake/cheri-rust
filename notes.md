# Running `x test`

We want to run `x test ...` to take advantage of the Rust test infrastructure and get some wide coverage. This builds and runs an executable for each test, and depends on having `std` compile for our target.

This branch contains the minimum changes needed to get this to a stage where it is compiling tests. To actually run the tests we need to produce executables that can run on the host machine or implement a `remote-test-server` which would handle this i.e. linking with the RTOS SDK and running through a simulator.

https://github.com/CHERIoT-Platform/cheri-rust/compare/beta...jacobpake:cheri-rust:x-test-hack

As well as these changes, you also need the following:

In `bootstrap.toml` remove `no-std = true` and add the dummy linker e.g. `linker = ./cheri/dummy-linker.sh` under our target.

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

There were a few assertions failing related to scalar ranges. I hope this is due to the use of `in_memory_size` instead of `capacity` [here](https://github.com/jacobpake/cheri-rust/blob/37d11537aecb490f57651e545b6b52a4b5851dce/compiler/rustc_abi/src/layout.rs#L1025).

## Output

Running:

```
x test library/core --target riscv32cheriot-unknown-cheriotrtos
```

Yielding:

```
	... compile failures ...

    library/core/src/result.rs - result::Result<T,E>::expect_err (line 1286)
    library/core/src/result.rs - result::Result<T,E>::expect (line 1152)
    library/core/src/result.rs - result::Result<T,E>::expect (line 1162)
    library/core/src/result.rs - result::Result<T,E>::unwrap (line 1220)
    library/core/src/result.rs - result::Result<T,E>::unwrap_err (line 1312)
    library/core/src/slice/index.rs - slice::index::range (line 874)
    library/core/src/slice/index.rs - slice::index::range (line 890)
    library/core/src/slice/index.rs - slice::index::range (line 882)

test result: FAILED. 5204 passed; 284 failed; 44 ignored; 0 measured; 0 filtered out; finished in 13.80s
```

A "pass" here means that it compiled and ran a dummy executable which always succeeds. Many of the failures are tests marked `should_panic`.
