// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

#![cfg(test)]

use target_feature_dispatch::target_feature_dispatch;

#[cfg(not(feature = "enable"))]
#[test]
fn fail() {
    panic!("This test program must be pre-configured using cargo-make files.");
}

#[test]
fn test_arm_features() {
    let is_neon: Option<bool> = target_feature_dispatch! {
        #[cfg_non_fallback(feature = "dispatch")]
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        if family("aarch64") {
            if "neon" {
                Some(true)
            } else {
                Some(false)
            }
        } else if "arm" {
            if "neon" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    };
    let is_dotprod: Option<bool> = target_feature_dispatch! {
        #[cfg_non_fallback(feature = "dispatch")]
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        if family("aarch64") {
            if "dotprod" {
                Some(true)
            } else {
                Some(false)
            }
        } else if "arm" {
            if "dotprod" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    };
    // Test feature detection results
    if cfg!(feature = "dispatch") && cfg!(any(target_arch = "aarch64", target_arch = "arm")) {
        if cfg!(feature = "target-arm-neon") {
            assert_eq!(
                is_neon,
                Some(true),
                "NEON / ASIMD must be available (mandatory on Rust + AArch64 ABI)."
            );
        } else {
            assert_eq!(is_neon, Some(false), "NEON / ASIMD must NOT be available.");
        }
        if cfg!(feature = "target-arm-dotprod") {
            assert_eq!(
                is_dotprod,
                Some(true),
                "Dot Product Instruction must be available."
            );
        } else {
            assert_eq!(
                is_dotprod,
                Some(false),
                "Dot Product Instruction must NOT be available."
            );
        }
    } else {
        assert!(is_neon.is_none(), "Fallback must be working.");
        assert!(is_dotprod.is_none(), "Fallback must be working.");
    }
}

#[test]
fn test_x86_features() {
    let is_sse2: Option<bool> = target_feature_dispatch! {
        #[cfg_non_fallback(feature = "dispatch")]
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        if family("x86") {
            if "sse2" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    };
    let is_avx2: Option<bool> = target_feature_dispatch! {
        #[cfg_non_fallback(feature = "dispatch")]
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        if family("x86") {
            if "avx2" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    };
    // Test feature detection results
    if cfg!(feature = "dispatch") && cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        // SSE2
        if cfg!(feature = "target-x86-sse2") {
            assert_eq!(is_sse2, Some(true), "SSE2 must be available.");
        } else {
            assert_eq!(is_sse2, Some(false), "SSE2 must NOT be available.");
        }
        // AVX2
        if cfg!(feature = "target-x86-avx2") {
            assert_eq!(is_avx2, Some(true), "AVX2 must be available.");
        } else {
            assert_eq!(is_avx2, Some(false), "AVX2 must NOT be available.");
        }
    } else {
        assert!(is_sse2.is_none(), "Fallback must be working.");
        assert!(is_avx2.is_none(), "Fallback must be working.");
    }
}

#[test]
fn test_wasm_features() {
    let is_simd128: Option<bool> = target_feature_dispatch! {
        #[cfg_non_fallback(feature = "dispatch")]
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        if class("wasm") {
            if "simd128" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    };
    // Test feature detection results
    if cfg!(feature = "dispatch") && cfg!(any(target_arch = "wasm32", target_arch = "wasm64")) {
        if cfg!(feature = "target-wasm-simd128") {
            assert_eq!(is_simd128, Some(true), "SIMD128 must be available.");
        } else {
            assert_eq!(is_simd128, Some(false), "SIMD128 must NOT be available.");
        }
    } else {
        assert!(is_simd128.is_none(), "Fallback must be working.");
    }
}
