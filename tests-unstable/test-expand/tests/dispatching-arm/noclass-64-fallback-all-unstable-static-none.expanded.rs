#![feature(stdarch_arm_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        {
                            {
                                #[cfg(not(all(target_feature = "dotprod")))] { "fallback" }
                            }
                        }
                    }
                }
            }
        }
    };
}
