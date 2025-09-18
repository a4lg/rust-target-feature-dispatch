// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

use std::sync::OnceLock;

use target_feature_dispatch::target_feature_dispatch;

// Make sure that opt_spec is used on every cases.
#[deny(unused_variables)]
fn test_opt_spec(opt_spec: &str) {
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    {
        #[cfg(target_feature = "sve2")]
        {
            assert_eq!(opt_spec, "Arm64+SVE2");
        }
        #[cfg(not(target_feature = "sve2"))]
        {
            assert_eq!(opt_spec, "Arm64+ASIMD");
        }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(target_feature = "avx2")]
        {
            assert_eq!(opt_spec, "x86+AVX2");
        }
        #[cfg(all(not(target_feature = "avx2"), target_feature = "sse3"))]
        {
            assert_eq!(opt_spec, "x86+SSE3");
        }
        #[cfg(not(any(target_feature = "avx2", target_feature = "sse3")))]
        {
            assert_eq!(opt_spec, "others");
        }
    }
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    {
        #[cfg(target_feature = "simd128")]
        {
            assert_eq!(opt_spec, "WebAssembly+SIMD");
        }
        #[cfg(not(target_feature = "simd128"))]
        {
            assert_eq!(opt_spec, "others");
        }
    }
    #[cfg(target_arch = "riscv64")]
    {
        assert_eq!(opt_spec, "RISC-V (64)");
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "wasm32",
        target_arch = "wasm64",
        target_arch = "riscv64"
    )))]
    {
        assert_eq!(opt_spec, "others");
    }
}

// Make sure that opt_spec is used on every cases.
#[deny(unused_variables)]
#[test]
fn usage_syntax_1() {
    // Arch+feature-specific paths: pure expression
    let opt_spec: &str = target_feature_dispatch! {
        #[static]
        if family("aarch64") {
            if "sve2" {
                "Arm64+SVE2"
            } else {
                "Arm64+ASIMD"
            }
        } else if family("x86") {
            if "avx2" {
                "x86+AVX2"
            } else if "sse3" {
                "x86+SSE3"
            }
        } else if "wasm32" || "wasm64" {
            if "simd128" {
                "WebAssembly+SIMD"
            }
        } else if "riscv64" {
            ( "RISC-V (64)" )
        } else {
            "others"
        }
    };
    test_opt_spec(opt_spec);
}

// Make sure that opt_spec is used on every cases.
#[deny(unused_variables)]
#[allow(unused_assignments)]
#[test]
fn usage_syntax_2() {
    // Preparation (arch-specific paths only)
    let mut opt_spec_base: String = target_feature_dispatch! {
        #[static]
        if family("aarch64") {
            ("Arm64".to_owned())
        } else if family("x86") {
            ("x86".to_owned())
        } else if "wasm32" || "wasm64" {
            ("WebAssembly".to_owned())
        } else if "riscv64" {
            ("RISC-V (64)".to_owned())
        } else {
            ("others".to_owned())
        }
    };
    // Arch+feature-specific paths: modify out_spec_base
    let opt_spec: &str = target_feature_dispatch! {
        #[static]
        if family("aarch64") {
            if "sve2" {
                opt_spec_base.push_str("+SVE2");
                opt_spec_base.as_str()
            } else {
                opt_spec_base.push_str("+ASIMD");
                opt_spec_base.as_str()
            }
        } else if family("x86") {
            if "avx2" {
                opt_spec_base.push_str("+AVX2");
                opt_spec_base.as_str()
            } else if "sse3" {
                opt_spec_base.push_str("+SSE3");
                opt_spec_base.as_str()
            } else {
                // Emulate fallback behavior of examples above.
                opt_spec_base = "others".to_owned();
                opt_spec_base.as_str()
            }
        } else if "wasm32" || "wasm64" {
            if "simd128" {
                opt_spec_base.push_str("+SIMD");
                opt_spec_base.as_str()
            } else {
                // Emulate fallback behavior of examples above.
                opt_spec_base = "others".to_owned();
                opt_spec_base.as_str()
            }
        } else if "riscv64" {
            // Although redundant (the same as the fallback),
            // keep the number of paths almost the same as examples above.
            // Also, use a block as an expression.
            { opt_spec_base.as_str() }
        } else {
            opt_spec_base.as_str()
        }
    };
    test_opt_spec(opt_spec);
}

// Make sure that opt_spec is used on every cases.
#[deny(unused_variables)]
#[allow(unused_assignments, unused_mut)]
#[test]
fn usage_syntax_3() {
    // Preparation (arch-specific paths only)
    let mut opt_spec: String = "others".to_owned();
    // Arch+feature-specific paths: assign to out_spec and does not return.
    target_feature_dispatch! {
        #[static]
        if family("aarch64") {
            if "sve2" {
                opt_spec = "Arm64+SVE2".to_owned();
            } else {
                opt_spec = "Arm64+ASIMD".to_owned();
            }
        } else if family("x86") {
            if "avx2" {
                opt_spec = "x86+AVX2".to_owned();
            } else if "sse3" {
                opt_spec = "x86+SSE3".to_owned();
            }
        } else if "wasm32" || "wasm64" {
            if "simd128" {
                opt_spec = "WebAssembly+SIMD".to_owned();
            }
        } else if "riscv64" {
            {
                opt_spec = "RISC-V (64)".to_owned();
            }
        } else if "loongarch64" {
            // Do nothing (as "loongarch64" is not on the list above).
        } else {
            // Do nothing.
        }
    };
    test_opt_spec(opt_spec.as_str());
}

/// Callback function used in [`combination_with_oncelock_1()`].
static CALLBACK_FUNC_1: OnceLock<fn() -> String> = OnceLock::new();

#[test]
fn combination_with_oncelock_1() {
    // fn (function pointer, called indirectly)
    let opt_spec = CALLBACK_FUNC_1.get_or_init(|| {
        target_feature_dispatch! {
            #[static]
            if family("aarch64") {
                if "sve2" {
                    fn sample() -> String {
                        "Arm64+SVE2".to_owned()
                    }
                    sample
                } else {
                    fn sample() -> String {
                        "Arm64+ASIMD".to_owned()
                    }
                    sample
                }
            } else if family("x86") {
                if "avx2" {
                    fn sample() -> String {
                        "x86+AVX2".to_owned()
                    }
                    sample
                } else if "sse3" {
                    fn sample() -> String {
                        "x86+SSE3".to_owned()
                    }
                    sample
                }
            } else if "wasm32" || "wasm64" {
                if "simd128" {
                    fn sample() -> String {
                        "WebAssembly+SIMD".to_owned()
                    }
                    sample
                }
            } else if "riscv64" {
                {
                    fn sample() -> String {
                        "RISC-V (64)".to_owned()
                    }
                    sample
                }
            } else {
                fn sample() -> String {
                    "others".to_owned()
                }
                sample
            }
        }
    })();
    test_opt_spec(opt_spec.as_str());
}

/// Callback function used in [`combination_with_oncelock_2()`].
static CALLBACK_FUNC_2: OnceLock<&'static (dyn Fn() -> String + Sync)> = OnceLock::new();

#[test]
fn combination_with_oncelock_2() {
    // Function reference as dynamic reference to Fn
    // (trait object; called *more* indirectly through vtable).
    let opt_spec = CALLBACK_FUNC_2.get_or_init(|| {
        target_feature_dispatch! {
            #[static]
            if family("aarch64") {
                if "sve2" {
                    fn sample() -> String {
                        "Arm64+SVE2".to_owned()
                    }
                    &sample
                } else {
                    fn sample() -> String {
                        "Arm64+ASIMD".to_owned()
                    }
                    &sample
                }
            } else if family("x86") {
                if "avx2" {
                    fn sample() -> String {
                        "x86+AVX2".to_owned()
                    }
                    &sample
                } else if "sse3" {
                    fn sample() -> String {
                        "x86+SSE3".to_owned()
                    }
                    &sample
                }
            } else if "wasm32" || "wasm64" {
                if "simd128" {
                    fn sample() -> String {
                        "WebAssembly+SIMD".to_owned()
                    }
                    &sample
                }
            } else if "riscv64" {
                {
                    fn sample() -> String {
                        "RISC-V (64)".to_owned()
                    }
                    &sample
                }
            } else {
                fn sample() -> String {
                    "others".to_owned()
                }
                &sample
            }
        }
    })();
    test_opt_spec(opt_spec.as_str());
}

/// Callback function used in [`combination_with_oncelock_3()`].
static CALLBACK_FUNC_3: OnceLock<&'static (dyn Fn() -> String + Sync)> = OnceLock::new();

#[test]
fn combination_with_oncelock_3() {
    // Closure as dynamic reference to Fn
    // (trait object; called *more* indirectly through vtable).
    let opt_spec = CALLBACK_FUNC_3.get_or_init(|| {
        target_feature_dispatch! {
            #[static]
            if family("aarch64") {
                if "sve2" {
                    &|| "Arm64+SVE2".to_owned()
                } else {
                    &|| "Arm64+ASIMD".to_owned()
                }
            } else if family("x86") {
                if "avx2" {
                    &|| "x86+AVX2".to_owned()
                } else if "sse3" {
                    &|| "x86+SSE3".to_owned()
                }
            } else if "wasm32" || "wasm64" {
                if "simd128" {
                    &|| "WebAssembly+SIMD".to_owned()
                }
            } else if "riscv64" {
                {
                    &|| "RISC-V (64)".to_owned()
                }
            } else {
                &|| "others".to_owned()
            }
        }
    })();
    test_opt_spec(opt_spec.as_str());
}

#[test]
fn rust_2024() {
    // Variant 1: `const` in a clause represents
    //            a ConstBlockExpression (Rust 2024 edition)
    const _RESULT_1: i32 = target_feature_dispatch! {
        #[static]
        if family("x86") {
            if "avx2" {
                const { 1 }
            } else {
                const { 2 }
            }
        } else {
            const { 3 }
        }
    };
    // Variant 2: `const` is only a beginning of statements.
    let _result2: i32 = target_feature_dispatch! {
        #[static]
        if family("x86") {
            if "avx2" {
                const A: i32 = 1;
                const B: i32 = 2;
                A + B
            } else {
                const A: i32 = 3;
                const B: i32 = 4;
                A + B
            }
        } else {
            const A: i32 = 5;
            const B: i32 = 6;
            A + B
        }
    };
}
