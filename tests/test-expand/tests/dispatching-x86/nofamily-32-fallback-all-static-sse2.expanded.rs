fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "x86"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        {
                            {
                                #[cfg(not(all(target_feature = "avx2")))]
                                {
                                    { #[cfg(all(target_feature = "sse2"))] { "x86 + SSE2" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
