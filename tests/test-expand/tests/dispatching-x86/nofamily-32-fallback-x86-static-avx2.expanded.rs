fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "x86"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        { { #[cfg(all(target_feature = "avx2"))] { "x86 + AVX2" } } }
                    }
                }
            }
        }
    };
}
