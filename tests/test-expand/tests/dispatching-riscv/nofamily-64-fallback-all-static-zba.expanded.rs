fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "riscv64"))]
                {
                    {
                        #[cfg(not(any(any(), feature = "detect-features")))]
                        { { #[cfg(all(target_feature = "zba"))] { "RISC-V + Zba" } } }
                    }
                }
            }
        }
    };
}
