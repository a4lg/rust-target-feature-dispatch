pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // ERROR: 0 is not an architecture.
    if 0 {
        (1)
    } else {
        0
    }
};
fn main() {}
