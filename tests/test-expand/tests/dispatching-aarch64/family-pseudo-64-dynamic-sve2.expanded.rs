fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
                { { #[cfg(all(target_feature = "sve2"))] { "AArch64 + SVE2" } } }
            }
        }
    };
}
