fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
                {
                    {
                        #[cfg(any(any(), feature = "detect-features"))]
                        {
                            if {
                                false || ::std_detect::detect::__is_feature_detected::sve2()
                            } {
                                "AArch64 + SVE2"
                            } else {
                                { #[cfg(not(all(target_feature = "sve2")))] { "fallback" } }
                            }
                        }
                    }
                }
            }
        }
    };
}
