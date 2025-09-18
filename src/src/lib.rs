// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

#![doc = include_str!("../docs/readme.md")]
// no_std by default.
#![no_std]
// Unsafe code is not allowed.  Forbid it unless in the tests and documentation.
#![deny(unsafe_code)]
#![cfg_attr(not(any(test, doc)), forbid(unsafe_code))]
// In the code maintenance mode, disallow all warnings.
#![cfg_attr(feature = "maint-code", deny(warnings))]
// Non-test code requires documents.
#![cfg_attr(not(test), warn(missing_docs, clippy::missing_docs_in_private_items))]
// Unless in the maintenance mode, allow unknown lints / old lint names.
#![cfg_attr(
    not(feature = "maint-lints"),
    allow(unknown_lints, renamed_and_removed_lints)
)]

// Import std on documentation.
#[cfg(doc)]
extern crate std;

#[doc = include_str!("../docs/target_feature_dispatch.md")]
#[macro_export]
macro_rules! target_feature_dispatch {
    /*
        Public Interface.

        Dynamic dispatching: default-disabled (any())
        Nightly features:    default-disabled (any())
        Non-fallback paths:  default-enabled  (all())
    */
    ($(#[$($pseudo_meta: tt)+])* $(if $($arch: tt $(($arch_arg: tt))?)||+ { $($if: tt)* })else+ else { $($else: tt)* }) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            (any(), any(), all())
            $(#[$($pseudo_meta)+])*
            ($($else)*) $((($($arch$(($arch_arg))?)||+) ($($if)*)))+
        )
    };

    /*
        Parse options.
    */
    // Unconditional "dynamic".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[dynamic] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_parse_options (all(), $nightly, $dispatch) $($rest)+)
    };
    // Unconditional "static".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[static] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_parse_options (any(), $nightly, $dispatch) $($rest)+)
    };
    // Unconditional "unstable".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[unstable] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_parse_options ($dyn, all(), $dispatch) $($rest)+)
    };
    // Unconditional "stable".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[stable] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_parse_options ($dyn, any(), $dispatch) $($rest)+)
    };
    // Conditional "dynamic".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[cfg_attr($meta: meta, dynamic)] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            (any($dyn, $meta), $nightly, $dispatch)
            $($rest)+
        )
    };
    // Conditional "static".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[cfg_attr($meta: meta, static)] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            (all($dyn, not($meta)), $nightly, $dispatch)
            $($rest)+
        )
    };
    // Conditional "unstable".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[cfg_attr($meta: meta, unstable)] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            ($dyn, any($nightly, $meta), $dispatch)
            $($rest)+
        )
    };
    // Conditional "stable".
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[cfg_attr($meta: meta, stable)] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            ($dyn, all($nightly, not($meta)), $dispatch)
            $($rest)+
        )
    };
    // Conditional non-fallback paths.
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[cfg_non_fallback($meta: meta)] $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_parse_options
            ($dyn, $nightly, all($dispatch, $meta))
            $($rest)+
        )
    };
    // Invalid pseudo-attribute.
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) #[$($pmeta: tt)+] $($rest: tt)+) => {
        compile_error!(concat!("invalid pseudo-attribute: ", stringify!(#[$($pmeta)+])));
    };
    // No more options (pass to the architecture-specific chain).
    (@__tgtfeat_dispatch_parse_options ($dyn: meta, $nightly: meta, $dispatch: meta) ($($else: tt)*) $(($($ifs: tt)+))+) => {
        {
            #[cfg($dispatch)]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($dyn, $nightly)
                    ($($else)*) $(($($ifs)+))+
                )
            }
            #[cfg(not($dispatch))]
            {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($else)*)
            }
        }
    };

    /*
        Architecture-specific `if`-`else` chain.
        Note that families are also parsed in @__tgtfeat_dispatch_arch_chain_2.
    */
    // `if`: family("aarch64") → any(target_arch = "aarch64", [target_arch = "arm64ec"])
    // Depend on the `arch-arm64ec` feature.
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((family("aarch64")) ($($if: tt)*)) $($rest: tt)*) => {
        $crate::__tgtfeat_dispatch_class_helper_arm64ec!(@__tgtfeat_dispatch_arch_chain ($($opts),*) ($($else)*) ((family("aarch64")) ($($if)*)) $($rest)*)
    };
    // `if`: family("riscv") → any(target_arch = "riscv32", target_arch = "riscv64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((family("riscv")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (family("riscv")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: family("x86") → any(target_arch = "x86", target_arch = "x86_64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((family("x86")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (family("x86")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: class("arm") → any(target_arch = "aarch64", [target_arch = "arm64ec"], target_arch = "arm")
    // Depend on the `arch-arm64ec` feature.
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("arm")) ($($if: tt)*)) $($rest: tt)*) => {
        $crate::__tgtfeat_dispatch_class_helper_arm64ec!(@__tgtfeat_dispatch_arch_chain ($($opts),*) ($($else)*) ((class("arm")) ($($if)*)) $($rest)*)
    };
    // `if`: class("mips") → any(target_arch = "mips", target_arch = "mips64", [target_arch = "mips32r6"], [target_arch = "mips64r6"])
    // Depend on the `arch-mips-r6` feature.
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("mips")) ($($if: tt)*)) $($rest: tt)*) => {
        $crate::__tgtfeat_dispatch_class_helper_mips_r6!(@__tgtfeat_dispatch_arch_chain ($($opts),*) ($($else)*) ((class("mips")) ($($if)*)) $($rest)*)
    };
    // `if`: class("mips-classic") → any(target_arch = "mips", target_arch = "mips64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("mips-classic")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("mips-classic")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "mips", target_arch = "mips64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: class("mipsr6") → any(target_arch = "mips32r6", target_arch = "mips64r6")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("mipsr6")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "mips32r6", target_arch = "mips64r6"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("mipsr6")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "mips32r6", target_arch = "mips64r6")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: class("powerpc") → any(target_arch = "powerpc", target_arch = "powerpc64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("powerpc")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("powerpc")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: class("sparc") → any(target_arch = "sparc", target_arch = "sparc64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("sparc")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("sparc")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "sparc", target_arch = "sparc64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: class("wasm") → any(target_arch = "wasm32", target_arch = "wasm64")
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("wasm")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("wasm")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `if`: Generic (others): pass to the final step below.
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) (($($arch: tt $(($arch_arg: tt))?)||+) ($($if: tt)*)) $($rest: tt)*) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (() ($($arch$(($arch_arg))?)||+) ($($if)*)) $($rest)*
        )
    };
    // `else`
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*)) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($else)*)
    };

    /*
        Architecture-specific `if`-`else` chain: `if` (final step)
        Conversion to regular list of architectures.
    */
    // family("aarch64") → "aarch64" || ["arm64ec"]
    // Depend on the `arch-arm64ec` feature.
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (family("aarch64") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::__tgtfeat_dispatch_class_helper_arm64ec!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)*) (family("aarch64") $(|| $($arch2 $(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // family("riscv") → "riscv32" || "riscv64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (family("riscv") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "riscv32", "riscv64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // family("x86") → "x86" || "x86_64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (family("x86") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "x86", "x86_64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("arm") → "aarch64" || ["arm64ec"] || "arm"
    // Depend on the `arch-arm64ec` feature.
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("arm") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::__tgtfeat_dispatch_class_helper_arm64ec!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)*) (class("arm") $(|| $($arch2 $(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("mips") → "mips" || "mips64" || ["mips32r6"] || ["mips64r6"]
    // Depend on the `arch-mips-r6` feature.
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("mips") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::__tgtfeat_dispatch_class_helper_mips_r6!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)*) (class("mips") $(|| $($arch2 $(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("mips-classic") → "mips" || "mips64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("mips-classic") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "mips", "mips64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("mipsr6") → "mips32r6" || "mips64r6"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("mipsr6") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "mips32r6", "mips64r6",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("powerpc") → "powerpc" || "powerpc64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("powerpc") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "powerpc", "powerpc64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("sparc") → "sparc" || "sparc64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("sparc") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "sparc", "sparc64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // class("wasm") → "wasm32" || "wasm64"
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("wasm") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "wasm32", "wasm64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // Others
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) ($arch1: tt ($arch1_arg: tt) $(|| $($arch2: tt$(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        compile_error!(concat!("Invalid architecture specifier: ", stringify!($arch1($arch1_arg))));
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) ($arch1: tt $(|| $($arch2: tt$(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* $arch1,) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    // Architectural branch conversion is completed.
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)+) () ($($if: tt)*)) $($rest: tt)*
    ) => {
        {
            #[cfg(any($(target_arch = $added),+))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause ($($added)||+) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any($(target_arch = $added),+)))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };

    /*
        Architecture-specific clause (one of architectural `if`).

        If the `else` clause is missing, fallback for static dispatching is of
        the root and fallback for dynamic dispatching is none
        (meaning, do the "last resort" static dispatching as a fallback).

        If the `else` clause exists, it is always used as the final fallback
        (for both static and dynamic dispatching methods).

        If one of the following is specified:

        1.  ( EXPR )
        2.  { /* statements (results in an expression) */ }
        3.  (empty)

        This is an architecture-only dispatch.
    */
    // `if`-`else` chain without `else`.
    (
        @__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*)
        ($(if $($feat: tt)&&+ { $($if: tt)* })else+)
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_entry
            ($($arch$(($arch_arg))?)||+) ($($opts),*)
            ($($else1)*) (@__tgtfeat_dispatch_no_fallback)
            $((($($feat)&&+) ($($if)*)))+
        )
    };
    // `if`-`else` chain with invalid `else`.
    (
        @__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*)
        ($(if $($feat: tt)&&+ { $($if: tt)* })else+ else { @__tgtfeat_dispatch_no_fallback })
    ) => {
        compile_error!("invalid feature-specific `else` clause");
    };
    // `if`-`else` chain with `else`.
    (
        @__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*)
        ($(if $($feat: tt)&&+ { $($if: tt)* })else+ else { $($else2: tt)* })
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_entry
            ($($arch$(($arch_arg))?)||+) ($($opts),*)
            ($($else2)*) ($($else2)*)
            $((($($feat)&&+) ($($if)*)))+
        )
    };
    // Architecture-only dispatch: Single expression enclosed by parens.
    (@__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*) (($expr: expr))) => {
        $expr
    };
    // Architecture-only dispatch: Single block results in an expression.
    (@__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*) ({$($tt: tt)*})) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($tt)*)
    };
    // Architecture-only dispatch: Empty (handle as returning the unit value).
    (@__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*) ()) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr ())
    };
    // Invalid architecture clause.
    (@__tgtfeat_dispatch_arch_clause ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) ($($else1: tt)*) ($($tt: tt)*)) => {
        compile_error!("unsupported or invalid architecture clause");
    };

    /*
        Feature-specific `if`-`else` chain (entrypoint).
        Determine dispatching based on the architecture.

        $else_sta is used on fallback static dispatch path.
        $else_dyn is either fallback on dynamic dispatch path or
        special keyword "@__tgtfeat_dispatch_no_fallback" meaning none.
    */
    // Arm / AArch64 (64-bit): AArch64 + Arm64EC
    (@__tgtfeat_dispatch_feat_chain_entry (family("aarch64")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_aarch64_feature_detected) $($rest)+
        )
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("aarch64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_aarch64_feature_detected) $($rest)+
        )
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("arm64ec") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_aarch64_feature_detected) $($rest)+
        )
    };
    // RISC-V (32-bit and 64-bit)
    // Depend on the `stable-std-riscv` feature.
    (@__tgtfeat_dispatch_feat_chain_entry (family("riscv")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::__tgtfeat_dispatch_dispatch_helper_riscv!(($($opts),*) $($rest)+)
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("riscv32") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::__tgtfeat_dispatch_dispatch_helper_riscv!(($($opts),*) $($rest)+)
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("riscv64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::__tgtfeat_dispatch_dispatch_helper_riscv!(($($opts),*) $($rest)+)
    };
    // x86 (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (family("x86")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_x86_feature_detected) $($rest)+
        )
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("x86") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_x86_feature_detected) $($rest)+
        )
    };
    (@__tgtfeat_dispatch_feat_chain_entry ("x86_64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_x86_feature_detected) $($rest)+
        )
    };
    // Arm (32-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("arm") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_arm_feature_detected) $($rest)+
        )
    };
    // LoongArch (64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("loongarch64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_loongarch_feature_detected) $($rest)+
        )
    };
    // MIPS (32-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("mips") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_mips_feature_detected) $($rest)+
        )
    };
    // MIPS (64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("mips64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_mips64_feature_detected) $($rest)+
        )
    };
    // PowerPC (32-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("powerpc") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_powerpc_feature_detected) $($rest)+
        )
    };
    // PowerPC (64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("powerpc64") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_powerpc64_feature_detected) $($rest)+
        )
    };
    // s390x (z/Architecture starting with IBM zSeries; 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry ("s390x") ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_s390x_feature_detected) $($rest)+
        )
    };
    // Arm (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (class("arm")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // MIPS (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (class("mips")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // MIPS (32-bit and 64-bit) - classic variant
    (@__tgtfeat_dispatch_feat_chain_entry (class("mips-classic")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // MIPS (32-bit and 64-bit) - ISA Release 6
    (@__tgtfeat_dispatch_feat_chain_entry (class("mipsr6")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // PowerPC (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (class("powerpc")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // SPARC (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (class("sparc")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // WebAssembly (32-bit and 64-bit)
    (@__tgtfeat_dispatch_feat_chain_entry (class("wasm")) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };
    // Others (use static dispatching only)
    (@__tgtfeat_dispatch_feat_chain_entry ($($arch: tt $(($arch_arg: tt))?)||+) ($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_dispatch_static ($($opts),*) $($rest)+)
    };

    /*
        Feature-specific `if`-`else` chain (with specific dispatching).
    */
    // Dynamic dispatching (if enabled).
    (
        @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($dyn: meta, $nightly: meta) ($detect: path)
        ($($else_sta: tt)*) ($($else_dyn: tt)*) $($rest: tt)+
    ) => {
        {
            #[cfg($dyn)]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_feat_chain_dynamic
                    ($detect)
                    ($($else_sta)*) ($($else_dyn)*) $($rest)+
                )
            }
            #[cfg(not($dyn))]
            {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_static ($($else_sta)*) $($rest)+)
            }
        }
    };
    // Dynamic dispatching only on Nightly (and if enabled).
    (
        @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($dyn: meta, $nightly: meta) ($detect: path)
        ($($else_sta: tt)*) ($($else_dyn: tt)*) $($rest: tt)+
    ) => {
        {
            #[cfg(all($dyn, $nightly))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_feat_chain_dynamic
                    ($detect)
                    ($($else_sta)*) ($($else_dyn)*) $($rest)+
                )
            }
            #[cfg(not(all($dyn, $nightly)))]
            {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_static ($($else_sta)*) $($rest)+)
            }
        }
    };
    // Static (only) dispatching.
    (
        @__tgtfeat_dispatch_feat_chain_dispatch_static ($dyn: meta, $nightly: meta)
        ($($else_sta: tt)*) ($($else_dyn: tt)*) $($rest: tt)+
    ) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_feat_chain_static ($($else_sta)*) $($rest)+)
    };

    /*
        Feature-specific dynamic dispatching.
    */
    // No feature-specific fallback
    // (use static dispatching with global fallback).
    (
        @__tgtfeat_dispatch_feat_chain_dynamic ($detect: path)
        ($($else_sta: tt)*) (@__tgtfeat_dispatch_no_fallback) $((($($feat: tt)&&+) ($($if: tt)*)))+
    ) => {
        $(
            if $({$detect!($feat)})&&+ {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($if)*)
            }
        )else+
        else {
            $crate::target_feature_dispatch!(
                @__tgtfeat_dispatch_feat_chain_static
                ($($else_sta)*) $((($($feat)&&+) ($($if)*)))+
            )
        }
    };
    // Architecture-specific fallback is specified
    // (always use local fallback).
    (
        @__tgtfeat_dispatch_feat_chain_dynamic ($detect: path)
        ($($else_sta: tt)*) ($($else_dyn: tt)*) $((($($feat: tt)&&+) ($($if: tt)*)))+
    ) => {
        $(
            if $({$detect!($feat)})&&+ {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($if)*)
            }
        )else+
        else {
            $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($else_dyn)*)
        }
    };

    /*
        Feature-specific static dispatching.
    */
    // `if`
    (@__tgtfeat_dispatch_feat_chain_static ($($else: tt)*) (($($feat: tt)&&+) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(all($(target_feature = $feat),+))]
            {
                $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($if)*)
            }
            #[cfg(not(all($(target_feature = $feat),+)))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_feat_chain_static
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    // `else`
    (@__tgtfeat_dispatch_feat_chain_static ($($else: tt)*)) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr $($else)*)
    };

    // Coerce tokens into an expression.
    (@__tgtfeat_dispatch_as_expr $expr: expr) => { $expr };
    // If empty, substitute with the unit value.
    (@__tgtfeat_dispatch_as_expr) => { () };
    // If the next coercion (STMTS → { STMTS }) did not make an expression,
    // raise a compile error.
    (@__tgtfeat_dispatch_as_expr {$($tt: tt)+}) => {
        compile_error!(concat!("failed to parse { ", stringify!($($tt)+), "} as expression"));
    };
    // Coercion for series of statements (STMTS → { STMTS }).
    (@__tgtfeat_dispatch_as_expr $($tt: tt)+) => {
        $crate::target_feature_dispatch!(@__tgtfeat_dispatch_as_expr { $($tt)+ } )
    };
}

/// Dispatch helper for RISC-V (MSRV 1.78).
#[cfg(feature = "stable-std-riscv")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_dispatch_helper_riscv {
    (($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn ($($opts),*)
            (::std::arch::is_riscv_feature_detected) $($rest)+
        )
    }
}

/// Dispatch helper for RISC-V (MSRV 1.78).
#[cfg(not(feature = "stable-std-riscv"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_dispatch_helper_riscv {
    (($($opts: meta),*) $($rest: tt)+) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_feat_chain_dispatch_dyn_nightly ($($opts),*)
            (::std::arch::is_riscv_feature_detected) $($rest)+
        )
    }
}

/// Architecture Grouping helper for MIPS (MSRV 1.73).
#[cfg(feature = "arch-mips-r6")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_class_helper_mips_r6 {
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("mips")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "mips", target_arch = "mips64", target_arch = "mips32r6", target_arch = "mips64r6"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("mips")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "mips", target_arch = "mips64", target_arch = "mips32r6", target_arch = "mips64r6")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("mips") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "mips", "mips64", "mips32r6", "mips64r6",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
}

/// Architecture Grouping helper for MIPS (MSRV 1.73).
#[cfg(not(feature = "arch-mips-r6"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_class_helper_mips_r6 {
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("mips")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("mips")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "mips", target_arch = "mips64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("mips") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "mips", "mips64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
}

/// Architecture Grouping helper for Arm64EC (MSRV 1.78).
#[cfg(feature = "arch-arm64ec")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_class_helper_arm64ec {
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((family("aarch64")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (family("aarch64")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "aarch64", target_arch = "arm64ec")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("arm")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "arm"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("arm")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "arm")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (family("aarch64") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "aarch64", "arm64ec",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("arm") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "aarch64", "arm64ec", "arm",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
}

/// Architecture Grouping helper for Arm64EC (MSRV 1.78).
#[cfg(not(feature = "arch-arm64ec"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tgtfeat_dispatch_class_helper_arm64ec {
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((family("aarch64")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "aarch64"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (family("aarch64")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "aarch64")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (@__tgtfeat_dispatch_arch_chain ($($opts: meta),*) ($($else: tt)*) ((class("arm")) ($($if: tt)*)) $($rest: tt)*) => {
        {
            #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_clause (class("arm")) ($($opts),*)
                    ($($else)*) ($($if)*)
                )
            }
            #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
            {
                $crate::target_feature_dispatch!(
                    @__tgtfeat_dispatch_arch_chain ($($opts),*)
                    ($($else)*) $($rest)*
                )
            }
        }
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (family("aarch64") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "aarch64",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
    (
        @__tgtfeat_dispatch_arch_chain_2 ($($opts: meta),*) ($($else: tt)*)
        (($($added: tt,)*) (class("arm") $(|| $($arch2: tt $(($arch2_arg: tt))?)||+)?) ($($if: tt)*)) $($rest: tt)*
    ) => {
        $crate::target_feature_dispatch!(
            @__tgtfeat_dispatch_arch_chain_2 ($($opts),*) ($($else)*)
            (($($added,)* "aarch64", "arm",) ($($($arch2$(($arch2_arg))?)||+)?) ($($if)*))
            $($rest)*
        )
    };
}
