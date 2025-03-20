fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "s390x"))]
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
                        {
                            {
                                #[cfg(
                                    all(target_feature = "vector-packed-decimal-enhancement-2")
                                )] { "s390x + vector packed decimal enhancement (2)" }
                            }
                        }
                    }
                }
            }
        }
    };
}
