fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    {
                        #[cfg(any(any(), feature = "detect-features"))]
                        {
                            if {
                                false || ::std_detect::detect::__is_feature_detected::avx2()
                            } {
                                "x86 + AVX2"
                            } else if {
                                true || ::std_detect::detect::__is_feature_detected::sse2()
                            } {
                                "x86 + SSE2"
                            } else {
                                "x86 (fallback)"
                            }
                        }
                    }
                }
            }
        }
    };
}
