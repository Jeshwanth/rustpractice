fn main() {
    let rows: u32 = 15;
    let mut expected: Vec<Vec<u32>> = Vec::new();


    //  Build:
    //  1
    //  11
    //  111
    //  1111
    println!("Input\n\n", );
    for x in 1..rows + 1 {
        let mut dvec: Vec<u32> = Vec::new();
        for _ in 0..x {
            dvec.push(1);
        }
        expected.push(dvec);
        println!("{:?}", expected[x as usize - 1]);
    }

    //  Make:
    //  1
    //  11
    //  121
    //  1331
    println!("\n\nOutput\n\n", );
    for x in 0..rows {
        if expected[x as usize].len() <= 2 {
            println!("{:?}", expected[x as usize]);
            continue;
        } else {
            let num_of_changes = x - 1;
            for s in 1..num_of_changes + 1 {
                expected[x as usize][s as usize] = expected[x as usize - 1][s as usize - 1] +
                    expected[x as usize - 1][s as usize];
            }
            println!("{:?}", expected[x as usize]);
        }
    }
}
