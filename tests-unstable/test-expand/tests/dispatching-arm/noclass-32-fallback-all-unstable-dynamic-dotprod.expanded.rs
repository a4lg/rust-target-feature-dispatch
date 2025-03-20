#![feature(stdarch_arm_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "arm"))]
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
