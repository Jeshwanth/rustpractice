// Faulhaber's Formula
// Based on http://exercism.io/submissions/215cbe4d7ea6443da4d73689934390f5

pub fn square_of_sum(num: u32) -> u32
{
    (num * num * (num + 1) * (num + 1))/4
}


pub fn sum_of_squares(num: u32) -> u32
{
    (num * (2 * num * num + 3 * num + 1))/6
}


pub fn difference(num: u32) -> u32
{
    return square_of_sum(num) - sum_of_squares(num);
}
