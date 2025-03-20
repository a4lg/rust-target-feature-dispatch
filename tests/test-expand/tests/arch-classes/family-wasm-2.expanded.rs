fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(
                    not(
                        any(
                            target_arch = "avr",
                            target_arch = "wasm32",
                            target_arch = "wasm64",
                            target_arch = "csky"
                        )
                    )
                )] { "fallback" }
            }
        }
    };
}
