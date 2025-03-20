#![cfg_attr(feature = "unstable", feature(stdarch_powerpc_feature_detection))]
fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if class("powerpc") {
            if "altivec" {
                "PowerPC + AltiVec"
            }
        } else {
            "fallback"
        }
    };
}
