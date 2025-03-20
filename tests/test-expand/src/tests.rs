// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

#![cfg(test)]

use std::env;

use macrotest::expand_args;

#[cfg(not(feature = "enable"))]
#[test]
fn fail() {
    panic!("This test program must be pre-configured using cargo-make files.");
}

#[test]
fn arch_classes() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Use LoongArch64, which is not in any of those classes.
    // Inside "*-2.rs", we also use non-family "avr" and "csky" archs.
    expand_args(
        "tests/arch-classes/*.rs",
        &[
            "--target",
            "loongarch64-unknown-linux-musl",
            "--features",
            "dispatch",
        ],
    );
}

#[test]
fn arch_families() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Use LoongArch64, which is not in any of those families.
    // Inside "*-2.rs", we also use non-family "avr" and "csky" archs.
    expand_args(
        "tests/arch-families/*.rs",
        &[
            "--target",
            "loongarch64-unknown-linux-musl",
            "--features",
            "dispatch",
        ],
    );
}

#[test]
fn dispatch_behavior_on_aarch64() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per target.
    struct TargetSpec {
        filename: &'static str,
        target: &'static str,
    }
    const TARGET_SPECS: &[TargetSpec] = &[TargetSpec {
        filename: "64",
        target: "aarch64-unknown-linux-musl",
    }];
    for target in TARGET_SPECS {
        // Per dispatching method.
        struct DispatchSpec {
            filename: &'static str,
            feature_suffix: &'static str,
        }
        const DISPATCH_SPECS: &[DispatchSpec] = &[
            DispatchSpec {
                filename: "static",
                feature_suffix: "",
            },
            DispatchSpec {
                filename: "dynamic",
                feature_suffix: ",detect-features",
            },
        ];
        for dispatch in DISPATCH_SPECS {
            // Per target feature (on compilation).
            struct FeatureSpec {
                filename: &'static str,
                target_feature: &'static str,
            }
            const FEATURE_SPECS: &[FeatureSpec] = &[
                FeatureSpec {
                    filename: "none",
                    target_feature: "",
                },
                FeatureSpec {
                    filename: "sve2",
                    target_feature: "+sve2",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target {target} -C target-feature={feature}",
                            target = target.target,
                            feature = feature.target_feature,
                        ),
                    );
                }
                // Expand and test five files each.
                let features = format!("dispatch{suffix}", suffix = dispatch.feature_suffix);
                expand_args(
                    format!(
                        "tests/dispatching-aarch64/family-fallback-all-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-aarch64/family-pseudo-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-aarch64/nofamily-{target}-fallback-all-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
            }
        }
    }
}

#[test]
fn dispatch_behavior_on_riscv() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per target.
    struct TargetSpec {
        filename: &'static str,
        target: &'static str,
    }
    const TARGET_SPECS: &[TargetSpec] = &[TargetSpec {
        filename: "64",
        target: "riscv64gc-unknown-linux-musl",
    }];
    for target in TARGET_SPECS {
        // Per dispatching method.
        struct DispatchSpec {
            filename: &'static str,
            feature_suffix: &'static str,
        }
        const DISPATCH_SPECS: &[DispatchSpec] = &[
            DispatchSpec {
                filename: "static",
                feature_suffix: "",
            },
            DispatchSpec {
                filename: "dynamic",
                feature_suffix: ",detect-features",
            },
        ];
        for dispatch in DISPATCH_SPECS {
            // Per target feature (on compilation).
            struct FeatureSpec {
                filename: &'static str,
                target_feature: &'static str,
            }
            const FEATURE_SPECS: &[FeatureSpec] = &[
                FeatureSpec {
                    filename: "none",
                    target_feature: "",
                },
                FeatureSpec {
                    filename: "zba",
                    target_feature: "+zba",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target {target} -C target-feature={feature}",
                            target = target.target,
                            feature = feature.target_feature,
                        ),
                    );
                }
                // Expand and test five files each.
                let features = format!("dispatch{suffix}", suffix = dispatch.feature_suffix);
                expand_args(
                    format!(
                        "tests/dispatching-riscv/family-fallback-all-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-riscv/family-pseudo-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-riscv/nofamily-{target}-fallback-all-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
            }
        }
    }
}

#[test]
fn dispatch_behavior_on_x86() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per target.
    struct TargetSpec {
        filename: &'static str,
        target: &'static str,
    }
    const TARGET_SPECS: &[TargetSpec] = &[
        TargetSpec {
            filename: "32",
            target: "i586-unknown-linux-musl",
        },
        TargetSpec {
            filename: "64",
            target: "x86_64-unknown-linux-musl",
        },
    ];
    for target in TARGET_SPECS {
        // Per dispatching method.
        struct DispatchSpec {
            filename: &'static str,
            feature_suffix: &'static str,
        }
        const DISPATCH_SPECS: &[DispatchSpec] = &[
            DispatchSpec {
                filename: "static",
                feature_suffix: "",
            },
            DispatchSpec {
                filename: "dynamic",
                feature_suffix: ",detect-features",
            },
        ];
        for dispatch in DISPATCH_SPECS {
            // Per target feature (on compilation).
            struct FeatureSpec {
                filename: &'static str,
                target_feature: &'static str,
            }
            const FEATURE_SPECS: &[FeatureSpec] = &[
                FeatureSpec {
                    filename: "none",
                    target_feature: "",
                },
                FeatureSpec {
                    filename: "sse2",
                    target_feature: "+sse2",
                },
                FeatureSpec {
                    filename: "avx2",
                    target_feature: "+avx2",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target {target} -C target-feature={feature}",
                            target = target.target,
                            feature = feature.target_feature,
                        ),
                    );
                }
                // Expand and test three files each.
                let features = format!("dispatch{suffix}", suffix = dispatch.feature_suffix);
                expand_args(
                    format!(
                        "tests/dispatching-x86/family-fallback-all-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-x86/family-pseudo-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-x86/nofamily-{target}-fallback-all-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
            }
        }
    }
}

#[test]
fn dispatch_behavior_on_x86_additional() {
    // Test per dispatch.
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    expand_args(
        "tests/dispatching-x86/dispatch-0.rs",
        &["--target", "x86_64-unknown-linux-musl"],
    );
    expand_args(
        "tests/dispatching-x86/dispatch-1.rs",
        &[
            "--target",
            "x86_64-unknown-linux-musl",
            "--features",
            "dispatch",
        ],
    );
    // Per target.
    struct TargetSpec {
        filename: &'static str,
        target: &'static str,
    }
    const TARGET_SPECS: &[TargetSpec] = &[
        TargetSpec {
            filename: "32",
            target: "i586-unknown-linux-musl",
        },
        TargetSpec {
            filename: "64",
            target: "x86_64-unknown-linux-musl",
        },
    ];
    for target in TARGET_SPECS {
        // Per dispatching method.
        struct DispatchSpec {
            filename: &'static str,
            feature_suffix: &'static str,
        }
        const DISPATCH_SPECS: &[DispatchSpec] = &[
            DispatchSpec {
                filename: "static",
                feature_suffix: "",
            },
            DispatchSpec {
                filename: "dynamic",
                feature_suffix: ",detect-features",
            },
        ];
        for dispatch in DISPATCH_SPECS {
            // Per target feature (on compilation).
            struct FeatureSpec {
                filename: &'static str,
                target_feature: &'static str,
            }
            const FEATURE_SPECS: &[FeatureSpec] = &[
                FeatureSpec {
                    filename: "none",
                    target_feature: "",
                },
                FeatureSpec {
                    filename: "sse2",
                    target_feature: "+sse2",
                },
                FeatureSpec {
                    filename: "avx2",
                    target_feature: "+avx2",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target {target} -C target-feature={feature}",
                            target = target.target,
                            feature = feature.target_feature,
                        ),
                    );
                }
                // Expand and test two files each.
                let features = format!("dispatch{suffix}", suffix = dispatch.feature_suffix);
                expand_args(
                    format!(
                        "tests/dispatching-x86/family-fallback-x86-{target}-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
                expand_args(
                    format!(
                        "tests/dispatching-x86/nofamily-{target}-fallback-x86-{dispatch}-{feature}.rs",
                        target = target.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
            }
        }
    }
}
