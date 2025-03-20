fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
                {
                    {
                        #[cfg(any(any(), feature = "detect-features"))]
                        {
                            if {
                                true || ::std_detect::detect::__is_feature_detected::zba()
                            } {
                                "RISC-V + Zba"
                            } else {
                                { #[cfg(all(target_feature = "zba"))] { "RISC-V + Zba" } }
                            }
                        }
                    }
                }
            }
        }
    };
}
