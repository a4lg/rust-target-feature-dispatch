pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // ERROR: error("error") is not a supported argument syntax.
    if error("error") {
        (1)
    } else {
        0
    }
};
fn main() {}
