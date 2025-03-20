# `target_feature_dispatch` (as expression)

It provides target feature dispatch macro for use in the expression position.

Once you write the target feature-specific branches once, this macro
implements both dynamic and static dispatch routines
(normally, those two are written very differently).

This crate is particularly useful on x86 and AArch64 targets.

# When is this crate useful?

If you want to optimize the program depending on the target features and
you want to satisfy both:

*   The final binary is going to be distributed for various environment but
    with low minimum requirements (e.g. x86_64 with only SSE2 enabled) and
*   You want to write a `no_std` crate without dynamic dispatch capabilities,

You will need both static and dynamic dispatching.
However, because of huge differences between static and dynamic dispatching,
you needed to write both and that can be tedious sometimes.

...Until now.  This crate provides a macro to easily write both static and
dynamic dispatch routines depending on the target architecture / features.

**The static dispatching is the default** (`no_std`-friendly and faster if the
environment is known on compilation) but
you can **easily switch to dynamic dispatching**.

You can easily write a static dispatch using
[`cfg_if`](https://crates.io/crates/cfg-if) but there's a core difference:
this macro is specialized for the expression position.

For instance, you can write a program like this:

```rust
use target_feature_dispatch::target_feature_dispatch;

fn print_optimized_env() {
    // Expression position macro block!
    // (this is not possible in general purpose cfg_if!)
    let opt_spec = target_feature_dispatch! {
        // Static by default but you can enable dynamic dispatching just by one line.
        #[dynamic]
        // Instead, you can conditionally enable dynamic dispatching, too.
        // Example:
        // #[cfg_attr(feature = "detect-features", dynamic)]
        if family("aarch64") {
            if "sve2" {
                "Arm64 with SVE2"
            } else {
                // SVE2 is unavailable
                // (note that Rust ABI needs ASIMD / NEON).
                "Arm64 (with ASIMD / NEON)"
            }
        } else if family("x86") {
            if "avx2" {
                "x86 / x86_64 with AVX2"
            } else if "sse2" {
                // SSE2 is the baseline of x86_64.
                "x86 with SSE2 / x86_64"
            }
        } else if "wasm32" || "wasm64" { // alternative: class("wasm")
            if "simd128" {
                "WebAssembly with 128-bit SIMD"
            }
        } else {
            "none"
        }
    };
    println!("Optimized for: {opt_spec}");
}
```

In combination with `OnceLock`, it can be used to store
a function pointer depending on the target features to execute
dynamic dispatching only once.

But note that it may not be *that* fast because Rust's standard target
feature detection macros cache results and they will effectively turn into
a series of branches (and mis-prediction is less likely on the hot path).

Benchmark your code to test which is better.

# When is this crate *not* useful?

Generating arch-specific functions itself is not an objective of this crate.
Use Rust's standard ways to create various arch-specific functions you need
and dispatch them using this macro.

Also, fine tuning is where this macro is not good at.
There are many target features that are not possible to discover in runtime
and you cannot specify those as long as dynamic dispatches are enabled (you can
turn it off but this macro is not flexible enough for fine tuning where
branching with target features is not sufficient).

This macro is intended for generic purpose programs that need to be reasonably
fast but not necessarily optimal on all cases.

# When is this crate suited the best?

*   The final binary (either a library or an executable)
    is distributed with low minimum requirements (than average),
*   Switching between non-SIMD and (not too many) SIMD implementations
    is feasible and makes huge difference in performance and
*   No fine-tuning is required.

# Other Features

*   Choose when you enable non-fallback optimized paths  
    This is particularly useful if you want to have an option to have
    a crate `unsafe`-free (because most arch-specific intrinsics are unsafe).
*   Choose between dynamic and static dispatching  
    By default, only `no_std`-friendly static dispatching is enabled.
    But you have an option to conditionally / unconditionally enable
    dynamic dispatching.
*   Optional Nightly Rust support on dynamic dispatching  
    By default, it supports dynamic dispatching of x86, AArch64 and RISC-V
    (standard feature detection macros are stabilized for them).
    But if you stick to the Nightly, you will have a dynamic dispatch
    capability to more architectures like Arm (32-bit) and LoongArch.  
    **Warning:**
    no SemVer-compatible semantics are guaranteed for unstable Rust features
    and only the latest version is going to be tested.

They are all configured through pseudo-attributes before the first `if`
(see the macro documentation for details).

# Major Version Lines

Raising MSRV of this crate is considered as a breaking change (requiring
a major version bump per MSRV bump) because it hardly makes sense to add
architectures without writing fast paths on the user side.

Also, a major version line is released on an edition change as a baseline
of that Rust language edition.

To mitigate compatibility issues, some migration features (with MSRV higher
than `package.rust-version` declared at the manifest of this crate) are defined.

*   **Version 1** (Compatibility Release)  
    *   Edition: 2021
    *   MSRV: 1.60 (with migration features raising MSRV)
    *   Supported Platforms (on stable channel):
        *   x86 / x86_64
        *   AArch64  
            (Windows Arm64EC support requires `arch-arm64ec` feature with MSRV 1.78)
        *   RISC-V  
            (requires `stable-std-riscv` feature with MSRV 1.78)
    *   Other Notes:
        *   `class("mips")` expands differently depending on
            the `arch-mips-r6` feature (with MSRV 1.73)
*   **Version 2** (Compatibility Release)  
    *   Edition: 2021
    *   MSRV: 1.78
    *   Supported Platforms (on stable channel):
        *   x86 / x86_64
        *   AArch64 (including Windows Arm64EC)
        *   RISC-V
*   **Version 3** (Actively Maintained)  
    *   The code is completely the same as the version 2 (as of version 3.0.0).
    *   Edition: 2024
    *   MSRV: 1.85
    *   Supported Platforms (on stable channel):
        *   x86 / x86_64
        *   AArch64 (including Windows Arm64EC)
        *   RISC-V

Still, exact set of target features with runtime detection availability depends
on the Rust version and see feature detection macros under `std::arch`
and the source code of [`std_detect`](https://github.com/rust-lang/stdarch/tree/master/crates/std_detect)
for details.
