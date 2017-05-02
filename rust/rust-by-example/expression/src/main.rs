fn main() {

    let x = 5;

    let y = {

        let x_squared = x * x;
        let x_cubed = x * x * x;

        x_squared * x_cubed

    };

    println!("{:?}", y);

    let z = {
        y * 5
    };

    println!("{:?}", z);

}
