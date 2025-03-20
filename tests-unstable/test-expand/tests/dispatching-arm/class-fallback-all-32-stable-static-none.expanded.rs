fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(
                    any(
                        target_arch = "aarch64",
                        target_arch = "arm64ec",
                        target_arch = "arm"
                    )
                )] { { #[cfg(not(all(target_feature = "dotprod")))] { "fallback" } } }
            }
        }
    };
}
