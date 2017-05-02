use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {

        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }

    }

    fn stringfy(&self) -> String {

        match *self {
            Cons(head, ref tail) => format!("{} {}", head, tail.stringfy()),
            Nil => format!("Nil"),
        }

    }
}

fn out_info(list: &List) {
    println!("{}", list.len());
    println!("{}", list.stringfy());
}

fn main() {

    let mut list = List::new();

    out_info(&list);

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    out_info(&list);

}
