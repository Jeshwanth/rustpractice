pub fn square(s: u32) -> u64 {
    if s > 0 && s < 65
    {
        return 2u64.pow(s-1)
    }
    else
    {
        panic!("Square must be between 1 and 64");
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for x in 0..64
    {
        sum = sum + 2u64.pow(x);
    }
return sum;
}
