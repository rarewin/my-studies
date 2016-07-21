use std::thread;

#[no_mangle]
pub extern fn process() {

    let handles: Vec<_> = (0..20).map(|_| {

        thread::spawn(|| {

            let mut x: i64 = 0;

            for _ in 0..1_000_000_000 {

                for _ in 0..500_000_000 {
                    x += 1;
                }

            }

            x

        })

    }).collect();

    for h in handles {

        println!("Thread finished with count = {}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());

    }

}
