fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                {
                    {
                        #[cfg(
                            not(
                                all(
                                    any(any(), feature = "detect-features"),
                                    any(any(), feature = "unstable")
                                )
                            )
                        )] { { #[cfg(all(target_feature = "zba"))] { "RISC-V + Zba" } } }
                    }
                }
            }
        }
    };
}
