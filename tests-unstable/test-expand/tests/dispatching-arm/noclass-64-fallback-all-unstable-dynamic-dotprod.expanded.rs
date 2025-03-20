#![feature(stdarch_arm_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64"))]
                {
                    {
                        #[cfg(any(any(), feature = "detect-features"))]
                        {
                            if {
                                true
                                    || ::std_detect::detect::__is_feature_detected::dotprod()
                            } {
                                "Arm + DOTPROD"
                            } else {
                                {
                                    #[cfg(all(target_feature = "dotprod"))] { "Arm + DOTPROD" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
