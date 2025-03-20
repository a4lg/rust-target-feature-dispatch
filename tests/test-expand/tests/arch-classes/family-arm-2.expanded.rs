fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(
                    not(
                        any(
                            target_arch = "avr",
                            target_arch = "aarch64",
                            target_arch = "arm64ec",
                            target_arch = "arm",
                            target_arch = "csky"
                        )
                    )
                )] { "fallback" }
            }
        }
    };
}
