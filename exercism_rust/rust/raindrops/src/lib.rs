use std::str;

pub fn raindrops(x: i32) -> String {
    let mut v = Vec::new();

    let mut p = 0;
	let mut mul = 1;
    let mut done = false;
    let mut j = 0;
    let mut retString = String::new();
    while !done{
        if x == 1
        {
            return "1".to_string();
        }
        else if (x%mul == 0) && !(j == mul)
        {
            j = x/mul;
            v.push(mul);
            v.push(j);
            mul = mul + 1;
        }
        else if j <= mul
        {
            done = true;
        }
        else{
            mul = mul + 1;
        }
    }
    v.sort();
    println!("The length is {}",v.len() );

    if v.binary_search(&3).is_ok()
    {
        retString = String::from("Pling");
    }
    if v.binary_search(&5).is_ok()
    {
        let a = String::from("Plang");
       retString =  retString + &a;
    }
    if v.binary_search(&7).is_ok()
    {
        let a = String::from("Plong");
        retString =  retString + &a;
    }

    if retString.is_empty()
    {
        retString = x.to_string();
    }

    return retString;
}
