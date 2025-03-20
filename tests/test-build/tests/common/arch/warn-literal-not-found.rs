#[deny(unexpected_cfgs)]
pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // ERROR: "not_found_err" is not a supported architecture.
    if "not_found_err" {
        (1)
    } else {
        0
    }
};
fn main() {}
