fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(
                    not(
                        any(
                            target_arch = "avr",
                            target_arch = "mips",
                            target_arch = "mips64",
                            target_arch = "csky"
                        )
                    )
                )] { "fallback" }
            }
        }
    };
}
