#![feature(stdarch_loongarch_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "loongarch64"))]
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
                                false || ::std_detect::detect::__is_feature_detected::lvz()
                            } {
                                "LoongArch64 + LVZ"
                            } else {
                                { #[cfg(not(all(target_feature = "lvz")))] { "fallback" } }
                            }
                        }
                    }
                }
            }
        }
    };
}
