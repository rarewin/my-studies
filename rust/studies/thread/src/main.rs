use std::thread;
fn main() {

    let child1 = thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello!");
    }).unwrap();

    let child2 = thread::spawn(move || {
        println!("Kon-nichiwa!");
    });

    let child1_result = child1.join();
    let child2_result = child2.join();

    if !(child1_result.is_ok() && child2_result.is_ok()) {
        panic!("Error?");
    } else {
        println!("Bye!");
    }
}
