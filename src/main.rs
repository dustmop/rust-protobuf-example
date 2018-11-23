extern crate protobuf;

use protobuf::Message;
mod gen;

fn main() {
    let mut animal = gen::animal::Animal::new();
    animal.set_kind("cat".to_owned());
    animal.set_sound("meow".to_owned());
    animal.set_weight(7.2);

    let serialized :Vec<u8> = animal.write_to_bytes().unwrap();
    println!("{:?}", serialized);
}
