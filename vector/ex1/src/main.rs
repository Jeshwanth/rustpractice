fn main() {
    let mut v = Vec::new();

    v.push(3);
    v.push(2);
    v.push(1);
    v.sort();
    println!("Hello, world! {:?} { }",v.get(0), v.len());

	for x in &v {
			    println!("{}", x);
	}
}
