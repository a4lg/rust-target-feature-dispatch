fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        { { #[cfg(not(all(target_feature = "sve2")))] { "fallback" } } }
                    }
                }
            }
        }
    };
}
