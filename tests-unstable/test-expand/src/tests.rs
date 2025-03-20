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
fn dispatch_behavior_on_arm() {
    // Per stability.
    struct StabilitySpec {
        filename: &'static str,
        feature_suffix: &'static str,
    }
    const STABILITY_SPECS: &[StabilitySpec] = &[
        StabilitySpec {
            filename: "stable",
            feature_suffix: "",
        },
        StabilitySpec {
            filename: "unstable",
            feature_suffix: ",unstable",
        },
    ];
    for stability in STABILITY_SPECS {
        // Per target.
        struct TargetSpec {
            filename: &'static str,
            target: &'static str,
        }
        const TARGET_SPECS: &[TargetSpec] = &[
            TargetSpec {
                filename: "32",
                target: "armv7-unknown-linux-musleabihf",
            },
            TargetSpec {
                filename: "64",
                target: "aarch64-unknown-linux-musl",
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
                        filename: "dotprod",
                        target_feature: "+dotprod",
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
                    let features = format!(
                        "dispatch{suffix1}{suffix2}",
                        suffix1 = dispatch.feature_suffix,
                        suffix2 = stability.feature_suffix
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-arm/class-fallback-all-{target}-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-arm/class-pseudo-{target}-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-arm/noclass-{target}-fallback-all-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                }
            }
        }
    }
}

#[test]
fn dispatch_behavior_on_loongarch64() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per stability.
    struct StabilitySpec {
        filename: &'static str,
        feature_suffix: &'static str,
    }
    const STABILITY_SPECS: &[StabilitySpec] = &[
        StabilitySpec {
            filename: "stable",
            feature_suffix: "",
        },
        StabilitySpec {
            filename: "unstable",
            feature_suffix: ",unstable",
        },
    ];
    for stability in STABILITY_SPECS {
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
                    filename: "lvz",
                    target_feature: "+lvz",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target loongarch64-unknown-linux-musl -C target-feature={feature}",
                            feature = feature.target_feature
                        ),
                    );
                }
                // Expand and test one file each.
                let features = format!(
                    "dispatch{suffix1}{suffix2}",
                    suffix1 = dispatch.feature_suffix,
                    suffix2 = stability.feature_suffix
                );
                expand_args(
                    format!(
                        "tests/dispatching-loongarch64/fallback-all-{stability}-{dispatch}-{feature}.rs",
                        stability = stability.filename,
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
fn dispatch_behavior_on_powerpc() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per stability.
    struct StabilitySpec {
        filename: &'static str,
        feature_suffix: &'static str,
    }
    const STABILITY_SPECS: &[StabilitySpec] = &[
        StabilitySpec {
            filename: "stable",
            feature_suffix: "",
        },
        StabilitySpec {
            filename: "unstable",
            feature_suffix: ",unstable",
        },
    ];
    for stability in STABILITY_SPECS {
        // Per target.
        struct TargetSpec {
            filename: &'static str,
            target: &'static str,
        }
        const TARGET_SPECS: &[TargetSpec] = &[
            TargetSpec {
                filename: "32",
                target: "powerpc-unknown-linux-gnu",
            },
            TargetSpec {
                filename: "64",
                target: "powerpc64-unknown-linux-gnu",
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
                        // Explicitly disabled because of implicit AltiVec capabilities on "powerpc64"
                        // (although disabling it does not currently cause errors / warnings).
                        target_feature: "-altivec",
                    },
                    FeatureSpec {
                        filename: "altivec",
                        target_feature: "+altivec",
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
                    let features = format!(
                        "dispatch{suffix1}{suffix2}",
                        suffix1 = dispatch.feature_suffix,
                        suffix2 = stability.feature_suffix
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-powerpc/class-fallback-all-{target}-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-powerpc/class-pseudo-{target}-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                    expand_args(
                        format!(
                            "tests/dispatching-powerpc/noclass-{target}-fallback-all-{stability}-{dispatch}-{feature}.rs",
                            target = target.filename,
                            stability = stability.filename,
                            dispatch = dispatch.filename,
                            feature = feature.filename
                        ),
                        &["--features", features.as_str()],
                    );
                }
            }
        }
    }
}

#[test]
fn dispatch_behavior_on_s390x() {
    unsafe {
        env::set_var("RUSTFLAGS", "");
    }
    // Per stability.
    struct StabilitySpec {
        filename: &'static str,
        feature_suffix: &'static str,
    }
    const STABILITY_SPECS: &[StabilitySpec] = &[
        StabilitySpec {
            filename: "stable",
            feature_suffix: "",
        },
        StabilitySpec {
            filename: "unstable",
            feature_suffix: ",unstable",
        },
    ];
    for stability in STABILITY_SPECS {
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
                    filename: "vector-packed-decimal-enhancement-2",
                    target_feature: "+vector-packed-decimal-enhancement-2",
                },
            ];
            for feature in FEATURE_SPECS {
                // Set target feature.
                unsafe {
                    env::set_var(
                        "RUSTFLAGS",
                        format!(
                            "--target s390x-unknown-linux-gnu -C target-feature={feature}",
                            feature = feature.target_feature
                        ),
                    );
                }
                // Expand and test one file each.
                let features = format!(
                    "dispatch{suffix1}{suffix2}",
                    suffix1 = dispatch.feature_suffix,
                    suffix2 = stability.feature_suffix
                );
                expand_args(
                    format!(
                        "tests/dispatching-s390x/fallback-all-{stability}-{dispatch}-{feature}.rs",
                        stability = stability.filename,
                        dispatch = dispatch.filename,
                        feature = feature.filename
                    ),
                    &["--features", features.as_str()],
                );
            }
        }
    }
}
