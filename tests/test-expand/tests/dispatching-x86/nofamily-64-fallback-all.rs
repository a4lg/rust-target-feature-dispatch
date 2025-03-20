fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "x86_64" {
            if "avx2" {
                "x86 + AVX2"
            } else if "sse2" {
                "x86 + SSE2"
            }
        } else {
            "fallback"
        }
    };
}
