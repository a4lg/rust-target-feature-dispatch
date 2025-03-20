fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
                { "fallback" }
            }
        }
    };
}
