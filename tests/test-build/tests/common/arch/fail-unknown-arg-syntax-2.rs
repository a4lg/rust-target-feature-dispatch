pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // ERROR: 1(2) is not a supported argument syntax.
    if 1(2) {
        (1)
    } else {
        0
    }
};
fn main() {}
