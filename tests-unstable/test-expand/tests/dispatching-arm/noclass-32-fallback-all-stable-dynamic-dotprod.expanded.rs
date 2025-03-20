fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "arm"))]
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
                                #[cfg(all(target_feature = "dotprod"))] { "Arm + DOTPROD" }
                            }
                        }
                    }
                }
            }
        }
    };
}
