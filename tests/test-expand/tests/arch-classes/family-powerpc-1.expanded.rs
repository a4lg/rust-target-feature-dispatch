fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
                { "fallback" }
            }
        }
    };
}
