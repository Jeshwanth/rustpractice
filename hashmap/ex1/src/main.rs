use std::collections::HashMap;

#[derive(Debug)]
struct Atom {
    name: String,
    atomic_number: u32,
}

fn main() {
    let mut atomicsymbol = HashMap::new();

    atomicsymbol.insert(
        "Al",
        Atom {
            name: "Aluminium".to_string(),
            atomic_number: 13,
        },
    );

    println!("Al is {:?}", atomicsymbol.get(&"Al"));

}
