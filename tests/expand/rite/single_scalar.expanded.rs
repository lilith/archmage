use archmage::{ScalarToken, rite};
#[inline]
fn compute(_t: ScalarToken, data: &[f32; 4]) -> f32 {
    let _: () = <ScalarToken>::__ARCHMAGE_ASSERT_TIER_D141EACA;
    data.iter().sum()
}
fn main() {}
