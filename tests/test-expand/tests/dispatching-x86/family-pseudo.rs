fn sample() {
    let value = target_feature_dispatch::target_feature_dispatch! {
        #[cfg_attr(feature = "detect-features", dynamic)]
        #[cfg_attr(feature = "unstable", unstable)]
        #[cfg_non_fallback(feature = "dispatch")]
        if "x86" || "x86_64" { // Not using family("x86") to force static dispatching.
            if "avx2" {
                "x86 + AVX2"
            } else if "sse2" {
                "x86 + SSE2"
            } else {
                "x86 (fallback)"
            }
        } else {
            "fallback"
        }
    };
}
