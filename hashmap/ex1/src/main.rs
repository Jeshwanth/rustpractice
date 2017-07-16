use std::collections::HashMap;

#[derive(Debug)]
struct atom {
    name: String,
    atomic_number: u32,
}

fn main() {
    let mut atomicsymbol = HashMap::new();

    atomicsymbol.insert(
        "Al",
        atom {
            name: "Aluminium".to_string(),
            atomic_number: 13,
        },
    );

    println!("Al is {:?}", atomicsymbol.get(&"Al"));


}
