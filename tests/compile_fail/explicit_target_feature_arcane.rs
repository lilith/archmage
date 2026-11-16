#[archmage::arcane]
#[target_feature(enable = "avx2")]
fn illegal(_t: archmage::X64V3Token) {}

fn main() {}
