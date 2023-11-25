#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
#[derive(Debug)]
pub enum List2 {
    Empty,
    Elem(i32, Box<List2>),
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);

    let anotherList: List2 = List2::Elem(
        1,
        Box::new(List2::Elem(
            2,
            Box::new(List2::Elem(3, Box::new(List2::Empty))),
        )),
    );
    println!("{:?}", anotherList);
}
