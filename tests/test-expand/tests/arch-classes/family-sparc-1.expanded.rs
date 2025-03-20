fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "sparc", target_arch = "sparc64")))]
                { "fallback" }
            }
        }
    };
}
