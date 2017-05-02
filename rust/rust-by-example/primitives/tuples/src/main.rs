use std::fmt::{self, Formatter, Display};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{{ {:.}, {:.} }}\n{{ {:.}, {:.} }}",
               self.0,
               self.1,
               self.2,
               self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    println!("1st value of 1st tuple: {}", (tuple_of_tuples.0).0);
    println!("2nd value of 2nd tuple: {}", (tuple_of_tuples.1).1);

    let pair = (1, true);

    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true, false);

    let (a, b, c, _, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

}
