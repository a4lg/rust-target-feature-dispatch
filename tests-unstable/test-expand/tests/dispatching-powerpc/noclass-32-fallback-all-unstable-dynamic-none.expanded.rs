#![feature(stdarch_powerpc_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "powerpc"))]
                {
                    {
                        #[cfg(
                            all(
                                any(any(), feature = "detect-features"),
                                any(any(), feature = "unstable")
                            )
                        )]
                        {
                            if {
                                false
                                    || ::std_detect::detect::__is_feature_detected::altivec()
                            } {
                                "PowerPC + AltiVec"
                            } else {
                                {
                                    #[cfg(not(all(target_feature = "altivec")))] { "fallback" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
