#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;

#[derive(Serialize, Deserialize, Debug)]
enum Clike {
    One = 1,
    Two = 2,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    bar: Clike,
}

fn main() {
    let v = serde_xdr::to_bytes(&Clike::One);
    println!("{:?}", v);

    let v = serde_xdr::to_bytes(&Clike::Two);
    println!("{:?}", v);

    let s = Foo { bar: Clike::One };
    let v = serde_xdr::to_bytes(&s);
    println!("{:?}", v);
}
