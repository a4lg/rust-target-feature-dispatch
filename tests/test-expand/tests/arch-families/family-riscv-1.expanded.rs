fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
                { "fallback" }
            }
        }
    };
}
