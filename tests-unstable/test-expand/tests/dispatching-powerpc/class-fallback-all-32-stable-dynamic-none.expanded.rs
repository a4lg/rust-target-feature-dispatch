fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
                { { #[cfg(not(all(target_feature = "altivec")))] { "fallback" } } }
            }
        }
    };
}
