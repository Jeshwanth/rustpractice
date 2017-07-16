//Based on http://exercism.io/submissions/68fc84235ed4429a83d66d11f70a51e1

pub fn square(s: u32) -> u64 {
    if !(s > 0 && s < 65) {panic!("Square must be between 1 and 64");}
    1 << (s - 1)
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
