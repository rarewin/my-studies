type NanoSecond = u64;
type Inchi = u64;

fn main() {

    let nanoseconds: NanoSecond = 5;
    let inches: Inchi = 2;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

}
