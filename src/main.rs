#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;

#[derive(Serialize, Deserialize, Debug)]
enum Clike {
    One = 1,
    Two = 2,
}

fn main() {
    let v = serde_xdr::to_bytes(&Clike::One);
    println!("{:?}", v);

    let v = serde_xdr::to_bytes(&Clike::Two);
    println!("{:?}", v);
}
