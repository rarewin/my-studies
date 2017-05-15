fn main() {

    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner long: {}", long_lived_binding);
        println!("innner short: {}", short_lived_binding);

        let long_lived_binding = 5.3_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

}
