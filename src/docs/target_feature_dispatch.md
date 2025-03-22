Target feature dispatch block as an expression.

# Overview

This macro accepts two-layer (or optionally one-layer) special `if`-`else`
chains for architecture / feature-specific dispatching (two-layer) or
[architecture-specific static dispatching (one-layer)](#dispatching-only-by-architecture).

Unlike general purpose [`cfg_if!`](https://crates.io/crates/cfg-if) macro,
this macro is intended for use in the expression position.  So...

*   You can obtain the result of the dispatch
    (as you may have seen in [the documentation at the crate root](crate)).
*   However, all paths must return the same type and
*   You cannot use this macro if neither an expression nor a statement block
    (possibly evaluated as an expression) are allowed in that position.

*The same type* seems too restrictive but don't worry!
For instance, you may use the type [`()`](primitive@unit) which is "returned"
when no attempt to return a value is made.

That means, if you process something but pass results using other methods than
the result of this *expression*, you can just write a series of statements
with trailing semicolon.
You are also allowed to use this macro like a regular block in a function.

Some examples below show placeholders with just comments but
they are also valid as usage of this macro because not doing anything
in a block is equivalent to returning the unit value (with the unit type).

# Template

Though it just stores an integer to `result`, it outlines
what can we do using this macro.

```
#![allow(unsafe_code)]
use target_feature_dispatch::target_feature_dispatch;

let result: i32 = target_feature_dispatch! {
    /*
        Supported pseudo-attributes just before the first `if` clause
        (see the "Configuration" section for details):

        #[cfg_non_fallback([CFG])]
        #[{static,dynamic,stable,unstable}]
        #[cfg_attr([CFG], {static,dynamic,stable,unstable})]

        Here, dynamic dispatching is enabled because the default is
        static dispatching (only) which is `no_std`-friendly and faster
        if the environment is known on compilation.
    */
    #[dynamic]
    if family("x86") {
        // x86 or x86_64 (using family specifier)
        if "avx2" {
            // AVX2 implementation as an expression.
            1
        } else if "sse4.1" && "popcnt" {
            // Implementation with both SSE4.1 and POPCNT which uses `&&` operator
            // (wrapped by an unsafe block but still *an* expression).
            unsafe { 2 }
        } else {
            // Generic x86 implementation
            // as a block evaluated as an expression.
            const A: i32 = 1;
            const B: i32 = 2;
            A + B
        }
    } else if family("aarch64") {
        // AArch64 (regular AArch64 or Arm64EC using family specifier)
        if "neon" {
            // AArch64 + ASIMD (NEON) implementation
            // (note that ASIMD is the baseline for Rust AArch64 ABI).
            const A: i32 = 2;
            const B: i32 = 2;
            A * B
        }
        /*
            Missing `else` clause here is invalid in the expression position
            on standard Rust but in this macro, this is allowed and
            substituted by the evaluation of the fallback path(s).

            Instead, it does not allow implicit fallback
            (i.e. not "returning" a value in an `if` clause for fall-through).
        */
    } else if "arm" {
        // Arm (32-bit)
        if "neon" {
            /*
                Dynamic detection of target features on Arm (32-bit) is
                still experimental.  Still, this path is executed when
                NEON is explicitly enabled by compiler options
                (static dispatching is always active).

                Alternatively, you may turn on `unstable` option in this
                macro.  If you do that (and enable required unstable
                features), dynamic dispatching is performed on Arm (32-bit).
            */
            5
        }
    } else if "loongarch64" {
        // LoongArch (64-bit)
        /*
            This is the example you only dispatch by the architecture
            (and not by target features).

            To distinguish with regular if statements,
            you may use one of these at this position:

            1.  (EXPR)
            2.  { /* statements... (as expression) */ }
            3.  literally nothing!
        */
        (6)
    } else if "wasm32" || "wasm64" { // alternative: class("wasm")
        // Architecture position allows `||` operator but disables
        // dynamic dispatching.  This is fine on WebAssembly architectures
        // because there's no runtime feature detection.
        if "simd128" {
            7
        }
    } else {
        // This else clause is mandatory because
        // this is the special fallback path (expanded in most cases).
        8
    }
};
```

# Dispatch: Architectures and Features

The contents inside this macro (except [pseudo-attributes](#configuration))
is up to two-layer `if`-`else` chains where

1.  The first being the architecture dispatch and
2.  The second being the feature dispatch
    (optional; the fallback path always lacks this layer).

Note that `else` on the first layer (architecture dispatch) is mandatory while
optional on the second layer (feature dispatch).  If there's no `else` in the
feature dispatch, it normally falls back to the `else` clause of architecture
dispatch
(see [the "Behavior" section](#behavior) for details).

# Architectures

All `target_arch`s supported by the Rust compiler is also supported
by this macro.

When you specify the architecture, you may use `||` operator to match
multiple architectures but with one big caveat: using this operator
**disables dynamic dispatching on that architecture clause** even if
feature detection is technically possible.

To make dynamic dispatching possible and merge two (or more) similar
architectures, consider using family / class specifiers explained below.

## Family / Class Specifiers

This macro supports special *family specifiers* and *class specifiers* for
matching for multiple architectures.

A *family* preserves dynamic dispatching capabilities while a *class* forces
static dispatching (for different reasons).

Currently, following family / class specifiers are supported:

| Family / Class           | Arch 1    | Arch 2      | Arch 3     | Arch 4     | Dispatching        |
|:------------------------ |:--------- |:----------- |:---------- |:---------- |:------------------ |
| `family("aarch64")` [^1] | `aarch64` | `arm64ec`   |            |            | Maybe Dynamic      |
| `family("riscv")`        | `riscv32` | `riscv64`   |            |            | Maybe Dynamic      |
| `family("x86")`          | `x86`     | `x86_64`    |            |            | Maybe Dynamic      |
| `class("arm")` [^1]      | `aarch64` | `arm64ec`   | `arm`      |            | Static Forced      |
| `class("mips")` [^2]     | `mips`    | `mips64`    | `mips32r6` | `mips64r6` | Static Forced [^3] |
| `class("mips-classic")`  | `mips`    | `mips64`    |            |            | Static Forced      |
| `class("powerpc")`       | `powerpc` | `powerpc64` |            |            | Static Forced      |
| `class("sparc")`         | `sparc`   | `sparc64`   |            |            | Static Only        |
| `class("wasm")`          | `wasm32`  | `wasm64`    |            |            | Static Only        |

[^1]: Version 1 requires the `arch-arm64ec` feature to include `arm64ec` (MSRV: 1.78).
[^2]: Version 1 requires the `arch-mips-r6` feature to include `mips32r6` and `mips64r6` (MSRV: 1.73).
[^3]: Unlike `mips` and `mips64` (supporting dynamic feature dispatching),
`mips32r6` and `mips64r6` do not support dynamic feature detection (despite that
they are the latest MIPS architectures; ISA Release 6).

*   Dispatching: **Maybe Dynamic**  
    Dynamic dispatching can be enabled unless explicitly disabled or
    the `||` operator is used.
*   Dispatching: **Static Forced**  
    Although its members support dynamic dispatching unless otherwise noted,
    using the class specifier disables that and forces static dispatching
    (mainly because they don't share the feature detection macro).
*   Dispatching: **Static Only**  
    Although architectures can be grouped together (e.g. for sharing some
    intrinsics), they don't have any dynamic dispatching capabilities.

All other architectures are handled in the generic path.

Note that again, `"x86" || "x86_64"` and `family("x86")` are different.
The former syntax disables the dynamic dispatching while the latter one doesn't.

## Dispatching Only by Architecture

You may skip dispatching by features.

However, due to the fact that the primary purpose of this macro is
architecture + feature-based dispatching and the syntax is optimized for this,
you cannot write arbitrary expression in the architecture-specific position.

Instead, you may use one of those:

1.  Single expression enclosed by parens `(` and `)`,
2.  Single block evaluated as an expression (without attributes)
    enclosed by curly braces `{` and `}`, or
3.  Nothing (do nothing and [implicitly return the unit value](#overview)).

# Features

In the feature-specific dispatch (the second `if`-`else` chain layer),
you may specify most of target CPU features.

It also allows using the `&&` operator when more than one feature is necessary
for given optimized implementation.

For instance, `"sse4.1" && "popcnt"` on x86 indicates that both
SSE4.1 and the `POPCNT` instruction must be supported to match
specified feature-specific clause.

## Architectures with Dynamic Dispatching

### Stable

| Architecture | MSRV | Crate Requirements                                                  |
|:------------ | ----:|:------------------------------------------------------------------- |
| `x86`        | 1.60 | Version 1+                                                          |
| `x86_64`     | 1.60 | Version 1+                                                          |
| `aarch64`    | 1.60 | Version 1+                                                          |
| `arm64ec`    | 1.78 | Version 1 (`arch-arm64ec` to use `family("aarch64")`) or Version 2+ |
| `riscv32`    | 1.78 | Version 1 + `stable-std-riscv` or Version 2+                        |
| `riscv64`    | 1.78 | Version 1 + `stable-std-riscv` or Version 2+                        |

#### See also

*   [Family / Class Specifiers](#family--class-specifiers)
*   [Migration Features](#migration-features)

### Unstable

*   `arm`
*   `loongarch64`
*   `mips`
*   `mips64`
*   `powerpc`
*   `powerpc64`
*   `s390x`

Only the latest Nightly Rust compiler is going to be tested.

## Warning on Dynamic Dispatching

**Note that** the features supported by the `target_feature` configuration
and the standard feature detection macro for the target architecture may differ.

It comes with various reasons:

1.  There's no way to detect all target features on some architectures.  
    For instance, x86 has `CPUID` instruction for feature detection but on
    the other hand, WebAssembly does not have a direct feature detection
    method and to do the feature detection, it requires
    [a "try and error" method from outside](https://github.com/GoogleChromeLabs/wasm-feature-detect).  
    Some architectures have partial (e.g. RISC-V) or mostly complete
    (e.g. AArch64) feature detection method but not available on
    the user mode (RISC-V: M-mode required (normally firmware level),
    AArch64: EL1 required (normally OS level)).  
    The reason why dynamic dispatching on AArch64 and RISC-V is supported
    is because the operating system exposes the target feature set to
    the user mode program but beware that the feature set we can know is
    limited to the set that the operating system chose to notify.
2.  Some features can be detected at runtime but feature detection macro
    for that architecture does not implement it.  
    RISC-V's extensions constraining the minimum vector register size
    (`Zvl*b`; e.g. `Zvl256b` for 256-bit or larger) is an example of this
    (despite that the register size test itself is technically possible at
    runtime as long as we can test the existence of the vector extension(s),
    testing the register size is a kind of *fine tuning* due to the nature
    of RISC-V's vector extensions).
3.  The "feature" is not an actual target CPU feature.  
    `crt-static` on many targets is an example of this.

If you prefer, [disable dynamic dispatching](Self#dynamic--static-dispatching)
to use static-only features safely.

# Configuration

To pass configuration from your crate to this macro,
it provides some pseudo-attributes (parsed by this macro itself).

For migration features (backport of succeeding versions) to change expansion
of this macro, see [the "Migration Features" section](#migration-features).

## Dynamic / Static Dispatching

The primary purpose of this macro is (conditionally) enable dynamic
dispatching for binary distribution to wider audiences.

Unconditional `dynamic` and `static` pseudo-attributes and conditional
`#[cfg_attr(..., [dynamic|static])` syntax (note: this is not actual
`cfg_attr` attribute) controls the dispatching method to use.

The default is `static` but can be overridden (just like `allow` and
`deny` attributes controlling the linting behavior).

Multiple control pseudo-attributes are evaluated top to bottom.

```
#![allow(unsafe_code)]
use target_feature_dispatch::target_feature_dispatch;

target_feature_dispatch! {
#   /*
    #[static] // You may remove this line.
    #[cfg_attr(feature = "detect-features", dynamic)]
#   */
    if family("x86") {
        if "avx2" {
            unsafe { /* Unsafe SIMD implementation using AVX2. */ }
        } else if "sse2" {
            unsafe { /* Unsafe SIMD implementation using SSE2. */ }
        }
    } else {
        // Safe fallback implementation.
    }
};
```

In the example above, it first overrides the default dispatching method to
`static` explicitly and *then* conditionally re-enables dynamic dispatching
only when the `detect-features` feature is enabled.

In the real world scenario, you won't need the first
`#[static]` because `static` is the default dispatching mode.

This is also useful on `no_std` crates since dynamic dispatching requires
the Rust standard library (`std`).

## Conditional Enablement of Non-fallback Paths

Architecture / feature-specific code paths are usually unsafe (due to the use
of unsafe intrinsics).  However, there's a case where you want a crate to be
optionally `unsafe`-free.

In this case, the `cfg_non_fallback` pseudo-attribute will help you.

It works just like the standard `cfg` attribute except that it still enables
the fallback path (the last `else` clause) when configured out.

If multiple `cfg_non_fallback` pseudo-attributes are specified, all
configuration tests must pass to enable non-fallback paths (just like `cfg`).
For instance, specifying both `#[cfg_non_fallback(A)]` and
`#[cfg_non_fallback(B)]` is equivalent to `#[cfg_non_fallback(all(A, B))]`.

```
# /*
#![cfg_attr(not(feature = "simd"), forbid(unsafe_code))]
# */
use target_feature_dispatch::target_feature_dispatch;

target_feature_dispatch! {
#   /*
    #[cfg_non_fallback(feature = "simd")]
#   */
    if family("x86") {
        if "avx2" {
            unsafe { /* Unsafe SIMD implementation using AVX2. */ }
        } else if "sse2" {
            unsafe { /* Unsafe SIMD implementation using SSE2. */ }
        }
    } else {
        // Safe fallback implementation.
    }
};
```

In this case, non-fallback paths need the `simd` feature (possibly
documented as *unsafe*) to be executed.  If the `simd` feature is disabled,
only the fallback path will be expanded and executed.

## Unstable: Dynamic Dispatching with Nightly Rust Features

Some feature detection macros and some target features usable on a stable
feature detection macro are available but only when you use Nightly version of
the Rust compiler and enable certain experimental features.

This macro supports using those feature detection macros but disabled
by default.  But this behavior can be overridden.

*Note (Warning):* Unlike feature detection macros, the use of unstable
target features cannot be controlled within this macro because it will
require a large database of possibly supported target features.

Unconditional `stable` and `unstable` pseudo-attributes and conditional
`#[cfg_attr(..., [stable|unstable])` syntax (note: this is not actual
`cfg_attr` attribute) controls whether this macro uses experimental
feature detection macros when the dynamic dispatching is enabled.

The default is `stable` but can be overridden (just like `allow` and
`deny` attributes controlling the linting behavior).

Multiple control pseudo-attributes are evaluated top to bottom.

```
use target_feature_dispatch::target_feature_dispatch;

target_feature_dispatch! {
    #[dynamic]
#   /*
    #[cfg_attr(feature = "unstable", unstable)]
#   */
    if family("aarch64") {
        if "sve2" {
            // AArch64 + SVE2 implementation
        } else {
            // AArch64 + ASIMD (NEON) implementation
            // (note that ASIMD is the baseline for Rust AArch64 ABI).
        }
    } else if "arm" {
        if "neon" {
            // Arm (32-bit) + NEON implementation.
            // If the feature `unstable` is enabled,
            // dynamic dispatching is performed here.
        }
    } else {
        // Safe fallback implementation.
    }
};
```

In this example, dynamic dispatching on the AArch64 architecture is always
enabled (because it is stabilized).  However, because dynamic feature
detection of Arm (32-bit) is experimental, dynamic dispatching is enabled
only if the feature `unstable` is enabled.

Note that, static dispatching is always active and if the target is Arm
(32-bit) and compiler options enable NEON,
Arm (32-bit) + NEON implementation will be executed.

### Some Do-It-Yourself (DIY) Part

If you use this unstable feature, you have to do something yourself.
For instance, when you need unstable dynamic dispatching enabled when the
`unstable` feature is enabled, you will need to write like this:

```
# #![allow(rustdoc::invalid_rust_codeblocks)]
// Conditionally enable experimental feature detection macros on
// Arm (32-bit), MIPS (32/64-bits), PowerPC (32/64-bits),
// LoongArch (32/64-bits; Rust is supported only on 64-bit LA64),
// and s390x (z/Architecture starting with IBM zSeries; 64-bit).
# /*
#![cfg_attr(
    feature = "unstable",
    feature(
        stdarch_arm_feature_detection,
        stdarch_mips_feature_detection,
        stdarch_loongarch_feature_detection,
        stdarch_powerpc_feature_detection,
        stdarch_s390x_feature_detection
    )
)]
# */
// Conditionally enable features that can be detected
// using `std::arch::is_riscv_feature_detected` macro on RISC-V
// but not yet stabilized either by RISC-V or Rust
// (stdarch_riscv_feature_detection) and experimental
// `target_feature`s as supported in RISC-V (riscv_target_feature).
# /*
#![cfg_attr(
    feature = "unstable",
    feature(
        riscv_target_feature,
        stdarch_riscv_feature_detection
    )
)]
# */
```

### Warning: `unstable` is Unstable

Just the name says.

It will not have SemVer-compatible semantics so expect some
breakage when the Nightly compiler changes the specification.

# Behavior

1.  If non-fallback paths are disabled, only fallback path
    (the last `else` clause) is expanded and executed.  
    Otherwise, architecture clauses (matching current `target_arch`) are
    statically evaluated (as explained below).
2.  If there's no matching architectures, only fallback path
    (the last `else` clause) is expanded and executed.  
    Otherwise, only one of the matching architecture clauses (the first matching
    one) is expanded as explained below (the fallback path is optionally passed
    to the architecture clause for later fallback behavior).  
3.  If the architecture clause is [the architecture-only dispatch](#dispatching-only-by-architecture),
    given expression / block is expanded and executed.
4.  If dynamic dispatching is enabled, at least all feature-specific paths are
    expanded and all feature conditions are checked from the top.
    The first matching clause is executed.
    *   One of the following (the first matching one) is expanded (always) and
        executed only if none of the `if` conditions match:
        1.  `else` feature clause in the architecture clause (if any).
        2.  The result of the static dispatching below (unless something weird
            happens, the fallback path is expected for expansion and execution).
5.  If static dispatching is enabled, all feature conditions are checked
    from the top and the first matching clause is expanded and executed.
    *   If none of the `if` conditions match, one of following clauses
        (the first matching one) is expanded and executed:
        1.  `else` feature clause in the architecture clause (if any).
        2.  `else` architecture clause of the root `if`-`else` chain
            (the fallback path; mandatory).

# Migration Features

Raising MSRV of this crate is considered as a breaking change (requiring
a major version bump per MSRV bump) because it hardly makes sense to add
architectures without writing fast paths on the user side.

Still, there are some cases that adding migration features
**to this crate** make sense.

1.  Unstable feature detection macro is stabilized later or
2.  Member of family / class can be added on the specific Rust version.

Those migration features changes how the macro is expanded.
Note that all of them are backports of succeeding versions of this crate
and these features make no effect on such non-backported versions.

Those features on non-backported versions will be kept for at least three years
but may be removed thereafter (note: once migration features are added, they
are kept indefinitely for the same major version line to comply with SemVer;
the feature removal only occurs on a major version bump).

*   Migration Feature: `arch-mips-r6`  
    Adds `mips32r6` and `mips64r6` to `class("mips")`.
    *   MSRV: 1.73
    *   Feature Ineffective On: version 2 (with MSRV 1.78)
*   Migration Feature: `arch-arm64ec`  
    Adds `arm64ec` to `family("aarch64")` and `class("arm")`.
    *   MSRV: 1.78
    *   Feature Ineffective On: version 2 (with MSRV 1.78)
*   Migration Feature: `stable-std-riscv`  
    Handles dynamic feature detection macro for RISC-V as stabilized.
    *   MSRV: 1.78
    *   Feature Ineffective On: version 2 (with MSRV 1.78)

# Reserved Keywords

To implement internal rules, it reserves two tokens (`@` and an identifier name)
starting with `@__tgtfeat_dispatch_`.

# Additional Examples

See [the "Template" section](#template) for usage of this macro.

## Basic Dispatching

It shows more complete example of optionally dynamic dispatching on x86.

In the example below, two crate features (in *your* crate) is expected to be
used.

*   `simd`  
    Use unsafe SIMD intrinsics (otherwise, use safe implementation only).
*   `detect-features`  
    Detect target features on runtime (static dispatching when disabled).

```
# #![allow(unused)]
#![allow(dead_code, unsafe_code)]

# /*
#[cfg(all(feature = "simd", target_arch = "x86"))]
# */
# #[cfg(target_arch = "x86")]
use core::arch::x86::*;
# /*
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
# */
# #[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use target_feature_dispatch::target_feature_dispatch;

// Its safety is separately tested.
# /*
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
# */
# #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn process_data_with_avx2(src: &[u8], dst: &mut [u8]) { /* ... */ }

// Its safety is separately tested.
# /*
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
# */
# #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
#[inline]
unsafe fn process_data_with_sse2(src: &[u8], dst: &mut [u8]) { /* ... */ }

#[inline]
fn process_data_naive(src: &[u8], dst: &mut [u8]) { /* ... */ }

pub fn process_data(src: &[u8], dst: &mut [u8]) {
    assert_eq!(src.len(), dst.len());
    // Dispatch per call.
    target_feature_dispatch! {
# /*
        #[cfg_non_fallback(feature = "simd")]
        #[cfg_attr(feature = "dispatch-features", dynamic)]
# */
        if family("x86") {
            if "avx2" {
                unsafe {
                    process_data_with_avx2(src, dst);
                }
            } else if "sse2" {
                unsafe {
                    process_data_with_sse2(src, dst);
                }
            }
        } else {
            process_data_naive(src, dst);
        }
    }
}
```

## With `OnceLock`

This is basically the same example as above but with
[`OnceLock`](std::sync::OnceLock), you can store a function pointer
suitable for the target feature and run dynamic dispatching only once.

The example below outlines how to use [`OnceLock`](std::sync::OnceLock)
containing a function pointer on x86 along with this macro.

You are able to use `&'static (dyn Fn(&[u8], &mut [u8]) + Sync)` instead of
`fn(&[u8], &mut [u8])` if you prefer returning closures but it will cost
a time per calling because of the existence of vtables.

```
# #![allow(unused)]
#![allow(dead_code, unsafe_code)]

# /*
#[cfg(all(feature = "simd", target_arch = "x86"))]
# */
# #[cfg(target_arch = "x86")]
use core::arch::x86::*;
# /*
#[cfg(all(feature = "simd", target_arch = "x86_64"))]
# */
# #[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use std::sync::OnceLock;

use target_feature_dispatch::target_feature_dispatch;

// Its safety is separately tested.
# /*
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
# */
# #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn process_data_with_avx2(src: &[u8], dst: &mut [u8]) { /* ... */ }

// Its safety is separately tested.
# /*
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
# */
# #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
#[inline]
unsafe fn process_data_with_sse2(src: &[u8], dst: &mut [u8]) { /* ... */ }

fn process_data_naive(src: &[u8], dst: &mut [u8]) { /* ... */ }

static PROCESS_FUNC: OnceLock<fn(&[u8], &mut [u8])> = OnceLock::new();

pub fn process_data(src: &[u8], dst: &mut [u8]) {
    assert_eq!(src.len(), dst.len());
    PROCESS_FUNC.get_or_init(|| {
        // This closure is called only once.
        target_feature_dispatch! {
# /*
            #[cfg_non_fallback(feature = "simd")]
            #[cfg_attr(feature = "dispatch-features", dynamic)]
# */
            if family("x86") {
                if "avx2" {
                    // This safe wrapper is required because PROCESS_FUNC
                    // expects function pointer to be a safe function.
                    fn safe_wrapper(src: &[u8], dst: &mut [u8]) {
                        unsafe {
                            process_data_with_avx2(src, dst);
                        }
                    }
                    safe_wrapper
                } else if "sse2" {
                    // Likewise.
                    fn safe_wrapper(src: &[u8], dst: &mut [u8]) {
                        unsafe {
                            process_data_with_sse2(src, dst);
                        }
                    }
                    safe_wrapper
                }
            } else {
                process_data_naive
            }
        }
    })(src, dst);
}
```
