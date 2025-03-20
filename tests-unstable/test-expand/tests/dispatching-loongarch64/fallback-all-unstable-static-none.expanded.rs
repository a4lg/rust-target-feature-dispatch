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
                            not(
                                all(
                                    any(any(), feature = "detect-features"),
                                    any(any(), feature = "unstable")
                                )
                            )
                        )]
                        { { #[cfg(not(all(target_feature = "lvz")))] { "fallback" } } }
                    }
                }
            }
        }
    };
}
