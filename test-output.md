# Test failures

- Type error: 1
- Instruction selection: 13
- Missing libcall: 108
- Relocation bounds: 2
- Runtime TagViolation: 3
- Runtime unhandled: 29

## Type error

library/core/src/intrinsics/mod.rs:677 (intrinsics::transmute)

```sh
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   --> library/core/src/intrinsics/mod.rs:680:5
    |
680 |     std::mem::transmute::<&i32, usize>(ptr)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: source type: `&i32` (64 bits)
    = note: target type: `usize` (32 bits)

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0512`.
```

## TagViolation

library/core/src/iter/traits/iterator.rs:2443 (iter::traits::iterator::Iterator::try_for_each)

```sh
TagViolation(0x2) error at 0x8000f8fa (v:0 0x8000e200-0x80026a00 l:0x18800 o:0x0 p: G R-cgm- X- ---) (return address: 0xa (v:0 0x0-0x0 l:0x0 o:0x0 p: - ------ -- ---)), with capability register CTP(0x4): 0x8001daac (v:0 0x8001daa3-0x8001daad l:0xa o:0x0 p: G R-cgm- X- ---)
```

library/core/src/primitive_docs.rs:1779 (prim_fn)

```sh
TagViolation(0x2) error at 0x8000eb0a (v:0 0x8000e200-0x8001e800 l:0x10600 o:0x0 p: G R-cgm- X- ---) (return address: 0x8000eaee (v:1 0x8000e200-0x8001e800 l:0x10600 o:0x5 p: G R-cgm- X- ---)), with capability register CA1(0xb): 0x8000ea5e (v:0 0x8000ea00-0x8000ea00 l:0x0 o:0x0 p: - ------ -- ---)
```

library/core/src/sync/atomic.rs:1689 (sync::atomic::AtomicPtr<T>::into_inner)

```sh
TagViolation(0x2) error at 0x8000eb7c (v:0 0x8000e200-0x8001e900 l:0x10700 o:0x0 p: G R-cgm- X- ---) (return address: 0x8000eb4a (v:1 0x8000e200-0x8001e900 l:0x10700 o:0x5 p: G R-cgm- X- ---)), with capability register CA0(0xa): 0x8002103c (v:0 0x8002103c-0x80021040 l:0x4 o:0x0 p: - RWcgml -- ---)
```

## Relocation bounds

library/core/src/alloc/global.rs:22 (alloc::global::GlobalAlloc)

```sh
error: ld.lld: error: build/cheriot/cheriot/release/test_runner.compartment:(function __rustc::__rust_alloc: .text+0x5ea): relocation R_RISCV_CHERIOT_COMPARTMENT_SIZE out of range: 135168 is not in [0, 4095]; references 'rust_out::ALLOCATOR::ha2f5dc787630f00c'
```

library/core/src/num/dec2flt/mod.rs:202 (num::dec2flt::ParseFloatError)

```sh
error: ld.lld: error: build/cheriot/cheriot/release/test_runner.compartment:(function core::num::dec2flt::lemire::compute_float::h48537a6fff8d13e7: .text+0xa09a): relocation R_RISCV_CHERIOT_COMPARTMENT_SIZE out of range: 10416 is not in [0, 4095]; references 'core::num::dec2flt::table::POWER_OF_FIVE_128::h8146cdaa5d532f26'
```

## Instruction selection

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:491 (core_simd::vector::Simd<T,N>::gather_or)

```sh
0x13a8c9d10: c64 = add 0x13b081310, 0x13a8c9ca0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:511 (core_simd::vector::Simd<T,N>::gather_or_default)

```sh
0x15245c710: c64 = add 0x15225c710, 0x15245c6a0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:608 (core_simd::vector::Simd<T,N>::gather_ptr)

```sh
0x12e871710: c64 = add 0x12e86c400, 0x12e8716a0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:534 (core_simd::vector::Simd<T,N>::gather_select)

```sh
0x13717c910: c64 = add 0x1371630a0, 0x13717c8a0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:641 (core_simd::vector::Simd<T,N>::gather_select_ptr)

```sh
0x14b042110: c64 = add 0x14b03ce00, 0x14b0420a0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:568 (core_simd::vector::Simd<T,N>::gather_select_unchecked)

```sh
0x14ac65170: c64 = add 0x14aa43000, 0x14aa04f20
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:749 (core_simd::vector::Simd<T,N>::scatter)

```sh
0x14fb0b110: c64 = add 0x14f8e14c0, 0x14fb0b0a0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:856 (core_simd::vector::Simd<T,N>::scatter_ptr)

```sh
0x13006dd10: c64 = add 0x130068a00, 0x13006dca0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:771 (core_simd::vector::Simd<T,N>::scatter_select)

```sh
0x1240cad10: c64 = add 0x124107cc0, 0x1240caca0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:883 (core_simd::vector::Simd<T,N>::scatter_select_ptr)

```sh
0x14d20dd10: c64 = add 0x14d313e00, 0x14d20dca0
```

library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:802 (core_simd::vector::Simd<T,N>::scatter_select_unchecked)

```sh
0x128b31b90: c64 = add 0x10502eee0, 0x128b31650
```

library/core/src/num/f64.rs:1853 (f64::math::div_euclid)

```sh
0x13204e550: f64 = frem 0x132050f50, 0x13204f040
```

library/core/src/num/f64.rs:1885 (f64::math::rem_euclid)

```sh
0x160023510: f64 = frem 0x160025d50, 0x160023e40
```

## Unhandled runtime

```
library/core/src/hint.rs:235 (hint::spin_loop)
library/core/src/iter/traits/iterator.rs:796 (iter::traits::iterator::Iterator::for_each)
library/core/src/primitive_docs.rs:1752 (prim_fn)
library/core/src/result.rs:1473 (result::Result<T,E>::and_then)
library/core/src/sync/atomic.rs:198 (sync::atomic)
library/core/src/sync/atomic.rs:663 (sync::atomic::AtomicBool::from_mut_slice)
library/core/src/sync/atomic.rs:632 (sync::atomic::AtomicBool::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicI16::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicI16::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicI32::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicI32::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicI64::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicI64::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicI8::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicI8::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicIsize::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicIsize::get_mut_slice)
library/core/src/sync/atomic.rs:1650 (sync::atomic::AtomicPtr<T>::from_mut_slice)
library/core/src/sync/atomic.rs:1611 (sync::atomic::AtomicPtr<T>::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicU16::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicU16::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicU32::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicU32::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicU64::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicU64::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicU8::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicU8::get_mut_slice)
library/core/src/sync/atomic.rs:2846 (sync::atomic::AtomicUsize::from_mut_slice)
library/core/src/sync/atomic.rs:2811 (sync::atomic::AtomicUsize::get_mut_slice)
```


## Missing libcalls

```
__atomic_fetch_nand_1
__atomic_fetch_nand_2
__atomic_fetch_nand_4
__atomic_fetch_nand_8
__divti3
__modti3
__powidf2
__powisf2
__udivti3
__umodti3
ceil
ceilf
floor
floorf
fma
fmax
fmaxf
fmin
fminf
fmodf
rint
rintf
round
roundf
sqrt
sqrtf
trunc
truncf
```

```
library/core/src/../../portable-simd/crates/core_simd/src/simd/num/float.rs:194 (core_simd::simd::num::float::SimdFloat::reduce_max)
library/core/src/../../portable-simd/crates/core_simd/src/simd/num/float.rs:221 (core_simd::simd::num::float::SimdFloat::reduce_min)
library/core/src/cmp.rs:1188 (cmp::PartialOrd)
library/core/src/iter/traits/iterator.rs:3142 (iter::traits::iterator::Iterator::max)
library/core/src/iter/traits/iterator.rs:3178 (iter::traits::iterator::Iterator::min)
library/core/src/num/f32.rs:909 (f32::f32::max)
library/core/src/num/f32.rs:935 (f32::f32::min)
library/core/src/num/f32.rs:1657 (f32::math::ceil)
library/core/src/num/f32.rs:1855 (f32::math::div_euclid)
library/core/src/num/f32.rs:1628 (f32::math::floor)
library/core/src/num/f32.rs:1780 (f32::math::fract)
library/core/src/num/f32.rs:1919 (f32::math::powi)
library/core/src/num/f32.rs:1887 (f32::math::rem_euclid)
library/core/src/num/f32.rs:1685 (f32::math::round)
library/core/src/num/f32.rs:1719 (f32::math::round_ties_even)
library/core/src/num/f32.rs:1946 (f32::math::sqrt)
library/core/src/num/f32.rs:1750 (f32::math::trunc)
library/core/src/num/f64.rs:927 (f64::f64::max)
library/core/src/num/f64.rs:953 (f64::f64::min)
library/core/src/num/f64.rs:2014 (f64::math::cbrt)
library/core/src/num/f64.rs:1655 (f64::math::ceil)
library/core/src/num/f64.rs:1626 (f64::math::floor)
library/core/src/num/f64.rs:1778 (f64::math::fract)
library/core/src/num/f64.rs:1917 (f64::math::powi)
library/core/src/num/f64.rs:1683 (f64::math::round)
library/core/src/num/f64.rs:1717 (f64::math::round_ties_even)
library/core/src/num/f64.rs:1944 (f64::math::sqrt)
library/core/src/num/f64.rs:1748 (f64::math::trunc)
library/core/src/num/int_macros.rs:884 (num::i128::checked_div)
library/core/src/num/int_macros.rs:951 (num::i128::checked_div_euclid)
library/core/src/num/int_macros.rs:1019 (num::i128::checked_div_exact)
library/core/src/num/int_macros.rs:3473 (num::i128::checked_ilog)
library/core/src/num/int_macros.rs:3521 (num::i128::checked_ilog10)
library/core/src/num/int_macros.rs:1822 (num::i128::checked_isqrt)
library/core/src/num/int_macros.rs:3341 (num::i128::checked_next_multiple_of)
library/core/src/num/int_macros.rs:1124 (num::i128::checked_rem)
library/core/src/num/int_macros.rs:1190 (num::i128::checked_rem_euclid)
library/core/src/num/int_macros.rs:3252 (num::i128::div_ceil)
library/core/src/num/int_macros.rs:3126 (num::i128::div_euclid)
library/core/src/num/int_macros.rs:1061 (num::i128::div_exact)
library/core/src/num/int_macros.rs:1068 (num::i128::div_exact)
library/core/src/num/int_macros.rs:1072 (num::i128::div_exact)
library/core/src/num/int_macros.rs:3210 (num::i128::div_floor)
library/core/src/num/int_macros.rs:3394 (num::i128::ilog)
library/core/src/num/int_macros.rs:3445 (num::i128::ilog10)
library/core/src/num/int_macros.rs:3091 (num::i128::isqrt)
library/core/src/num/int_macros.rs:3297 (num::i128::next_multiple_of)
library/core/src/num/int_macros.rs:2783 (num::i128::overflowing_div)
library/core/src/num/int_macros.rs:2812 (num::i128::overflowing_div_euclid)
library/core/src/num/int_macros.rs:2841 (num::i128::overflowing_rem)
library/core/src/num/int_macros.rs:2870 (num::i128::overflowing_rem_euclid)
library/core/src/num/int_macros.rs:3163 (num::i128::rem_euclid)
library/core/src/num/int_macros.rs:3174 (num::i128::rem_euclid)
library/core/src/num/int_macros.rs:2030 (num::i128::saturating_div)
library/core/src/num/int_macros.rs:920 (num::i128::strict_div)
library/core/src/num/int_macros.rs:926 (num::i128::strict_div)
library/core/src/num/int_macros.rs:932 (num::i128::strict_div)
library/core/src/num/int_macros.rs:987 (num::i128::strict_div_euclid)
library/core/src/num/int_macros.rs:993 (num::i128::strict_div_euclid)
library/core/src/num/int_macros.rs:999 (num::i128::strict_div_euclid)
library/core/src/num/int_macros.rs:1159 (num::i128::strict_rem)
library/core/src/num/int_macros.rs:1165 (num::i128::strict_rem)
library/core/src/num/int_macros.rs:1171 (num::i128::strict_rem)
library/core/src/num/int_macros.rs:1225 (num::i128::strict_rem_euclid)
library/core/src/num/int_macros.rs:1231 (num::i128::strict_rem_euclid)
library/core/src/num/int_macros.rs:1237 (num::i128::strict_rem_euclid)
library/core/src/num/int_macros.rs:2175 (num::i128::wrapping_div)
library/core/src/num/int_macros.rs:2201 (num::i128::wrapping_div_euclid)
library/core/src/num/int_macros.rs:2227 (num::i128::wrapping_rem)
library/core/src/num/int_macros.rs:2252 (num::i128::wrapping_rem_euclid)
library/core/src/num/nonzero.rs:1382 (num::nonzero::NonZero<u128>::div_ceil)
library/core/src/num/nonzero.rs:1643 (num::nonzero::NonZero<u128>::ilog10)
library/core/src/num/nonzero.rs:1736 (num::nonzero::NonZero<u128>::isqrt)
library/core/src/num/saturating.rs:331 (num::saturating::Saturating<i128>)
library/core/src/num/saturating.rs:339 (num::saturating::Saturating<i128>)
library/core/src/num/saturating.rs:331 (num::saturating::Saturating<u128>)
library/core/src/num/saturating.rs:339 (num::saturating::Saturating<u128>)
library/core/src/num/uint_macros.rs:1196 (num::u128::checked_div)
library/core/src/num/uint_macros.rs:1251 (num::u128::checked_div_euclid)
library/core/src/num/uint_macros.rs:1305 (num::u128::checked_div_exact)
library/core/src/num/uint_macros.rs:1638 (num::u128::checked_ilog)
library/core/src/num/uint_macros.rs:1719 (num::u128::checked_ilog10)
library/core/src/num/uint_macros.rs:3597 (num::u128::checked_next_multiple_of)
library/core/src/num/uint_macros.rs:1395 (num::u128::checked_rem)
library/core/src/num/uint_macros.rs:1451 (num::u128::checked_rem_euclid)
library/core/src/num/uint_macros.rs:3541 (num::u128::div_ceil)
library/core/src/num/uint_macros.rs:1342 (num::u128::div_exact)
library/core/src/num/uint_macros.rs:1559 (num::u128::ilog)
library/core/src/num/uint_macros.rs:1610 (num::u128::ilog10)
library/core/src/num/uint_macros.rs:3624 (num::u128::is_multiple_of)
library/core/src/num/uint_macros.rs:3429 (num::u128::isqrt)
library/core/src/num/uint_macros.rs:3574 (num::u128::next_multiple_of)
library/core/src/num/uint_macros.rs:2328 (num::u128::saturating_div)
library/core/src/ops/arith.rs:627 (ops::arith::f128)
library/core/src/ops/arith.rs:627 (ops::arith::f16)
library/core/src/ops/arith.rs:627 (ops::arith::f32)
library/core/src/ops/arith.rs:627 (ops::arith::f64)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicI16::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicI32::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicI64::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicI8::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicIsize::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicU16::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicU32::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicU64::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicU8::fetch_nand)
library/core/src/sync/atomic.rs:3299 (sync::atomic::AtomicUsize::fetch_nand)
library/core/src/time.rs:319 (time::Duration::from_nanos_u128)
```
