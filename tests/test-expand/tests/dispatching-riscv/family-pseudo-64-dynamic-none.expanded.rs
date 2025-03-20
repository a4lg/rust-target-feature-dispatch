fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                { { #[cfg(not(all(target_feature = "zba")))] { "fallback" } } }
            }
        }
    };
}
