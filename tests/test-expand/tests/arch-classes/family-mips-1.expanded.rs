fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(
                    not(
                        any(
                            target_arch = "mips",
                            target_arch = "mips64",
                            target_arch = "mips32r6",
                            target_arch = "mips64r6"
                        )
                    )
                )] { "fallback" }
            }
        }
    };
}
