#![feature(stdarch_arm_feature_detection)]
fn sample() {
    let value = {
        #[cfg(all(all(), feature = "dispatch"))]
        {
            {
                #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
                { { #[cfg(all(target_feature = "dotprod"))] { "Arm + DOTPROD" } } }
            }
        }
    };
}
