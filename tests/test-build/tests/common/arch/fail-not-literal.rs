pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // ERROR: (()) is not an architecture.
    if (()) {
        (1)
    } else {
        0
    }
};
fn main() {}
