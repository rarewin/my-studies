use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {

    let xs = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("xs[0]: {}", xs[0]);
    println!("ys[499]: {}", ys[499]);

    println!("len of xs: {}", xs.len());
    println!("len of ys: {}", ys.len());

    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("array ys occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("{:?}", xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    println!("{:?}", &ys[1..4]);

    // println!("{}", xs[5]); // panic!
}
