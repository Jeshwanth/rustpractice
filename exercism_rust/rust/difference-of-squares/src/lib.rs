

pub fn square_of_sum(num: u32) -> u32
{
    let mut sum = 0;
    let sqrofsum;
    for x in 1..num+1 {
    sum = sum + x;
    }
    sqrofsum = sum * sum;
    return sqrofsum;
}


pub fn sum_of_squares(num: u32) -> u32
{
    let mut sumofsquare = 0;
    for x in 1..num+1 {
    sumofsquare = sumofsquare + (x * x);
    }
    return sumofsquare;
}


pub fn difference(num: u32) -> u32
{
    return square_of_sum(num) - sum_of_squares(num);
}
