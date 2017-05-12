struct Unittype;

struct Tupletype(i32,i32);

struct Fieldtype { x: i32, y: i32}

struct Structtype {
    a: Fieldtype,
    b: Fieldtype,
}

fn main() {
    println!("Learning struct!");

    let _unittype = Unittype;
    let _tupletype = Tupletype(3,4);
    let _fieldtype = Fieldtype{ x: 77, y: 55};

    println!("fieldtype x is {}",_fieldtype.x );
}
