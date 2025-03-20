fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                { { #[cfg(all(target_feature = "avx2"))] { "x86 + AVX2" } } }
            }
        }
    };
}
