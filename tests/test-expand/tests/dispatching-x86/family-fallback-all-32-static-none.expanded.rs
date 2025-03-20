fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        {
                            {
                                #[cfg(not(all(target_feature = "avx2")))]
                                {
                                    { #[cfg(not(all(target_feature = "sse2")))] { "fallback" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
