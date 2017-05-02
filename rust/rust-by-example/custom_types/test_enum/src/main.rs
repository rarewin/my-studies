#![allow(dead_code)]

enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {

    match p {
        Person::Skinny => println!("Is skinny!"),
        Person::Fat => println!("Is fat!"),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => println!("{} is {} tall!", name, height),
    }
}

fn main() {
    inspect(Person::Height(18));
    inspect(Person::Weight(10));
    inspect(Person::Fat);
    inspect(Person::Skinny);
    inspect(Person::Info {
                name: "Dave".to_owned(),
                height: 72,
            });
}
