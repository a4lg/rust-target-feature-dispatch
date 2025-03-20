fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "mips", target_arch = "mips64")))]
                { "fallback" }
            }
        }
    };
}
