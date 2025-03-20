#![feature(stdarch_s390x_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "s390x"))]
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
                                    || ::std_detect::detect::__is_feature_detected::vector_packed_decimal_enhancement_2()
                            } {
                                "s390x + vector packed decimal enhancement (2)"
                            } else {
                                {
                                    #[cfg(
                                        not(
                                            all(target_feature = "vector-packed-decimal-enhancement-2")
                                        )
                                    )] { "fallback" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
