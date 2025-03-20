fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
                { "fallback" }
            }
        }
    };
}
