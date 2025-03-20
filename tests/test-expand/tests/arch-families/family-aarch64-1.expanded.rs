fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "aarch64", target_arch = "arm64ec")))]
                { "fallback" }
            }
        }
    };
}
