#![cfg_attr(feature = "unstable", feature(stdarch_s390x_feature_detection))]
fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "s390x" {
            if "vector-packed-decimal-enhancement-2" {
                "s390x + vector packed decimal enhancement (2)"
            }
        } else {
            "fallback"
        }
    };
}
