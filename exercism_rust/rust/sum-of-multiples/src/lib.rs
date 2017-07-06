

pub fn sum_of_multiples(num: i32, list:  &Vec<i32>) -> i32
{
    let mut listtwo = Vec::new();
    let length = list.len();
    for x in 1..num {
        for y in list {
                if x % y == 0
                {
                    listtwo.push(x);
                    break;
                }
                else
                {
                    continue;
                }
        }
    }

    if listtwo.len() == 0
    {
        return 0
    }
    else
    {
        let sum: i32 = listtwo.iter().sum();
        return sum;
    }

}
