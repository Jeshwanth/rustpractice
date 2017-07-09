fn main() {
    println!("Test 2D Vector!");
    let mut expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];

    println!("let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];");
    println!("expected.len() is {}", expected.len());
    println!("expected[0].len() is {}", expected[0].len());
    println!("expected[1].len() is {}", expected[1].len());
    println!("expected[2].len() is {}", expected[2].len());

    println!("expected[0] is {:?}", expected[0]);
    println!("expected[1] is {:?}", expected[1]);
    println!("expected[2] is {:?}", expected[2]);

    println!("expected[0][0] is {}", expected[0][0]);
    println!("expected[1][0] is {}", expected[1][0]);
    println!("expected[1][1] is {}", expected[1][1]);
    println!("expected[2][0] is {}", expected[2][0]);
    println!("expected[2][1] is {}", expected[2][1]);
    println!("expected[2][2] is {}", expected[2][2]);

    expected[0].push(3);
    println!("After expected[0].push(3);");
    println!("expected[0].len() is {}", expected[0].len());
    println!("expected[0] is {:?}", expected[0]);

}
