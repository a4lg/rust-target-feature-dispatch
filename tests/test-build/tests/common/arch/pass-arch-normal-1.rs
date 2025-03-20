pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    if "x86" {
        (1)
    } else {
        0
    }
};
fn main() {}
