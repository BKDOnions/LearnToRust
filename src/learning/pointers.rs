use std::ops::Deref;
use std::rc::Rc;

use LinkedList::{Cons, Nil};

#[derive(Debug)]
enum LinkedList {
    Cons(i32, Rc<LinkedList>),
    Nil,
}

fn create_linked_list() -> (LinkedList, LinkedList) {
    let list = Rc::new(Cons(
        1,
        Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil))))))),
    ));
    let list1 = Cons(11, Rc::clone(&list));
    let list2 = Cons(10, Rc::clone(&list));
    (list1, list2)
}

fn ref_test() {
    let x = 5;
    let y = &x;
    let z = &y;
    assert_eq!(y, *z);
    assert_eq!(x, *y);
}

fn deref_test() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(x, *y);
}

pub fn first_box() {
    let b = Box::new(5);
    println!("{}", b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod pointer_tests {
    use std::rc::Rc;

    use crate::learning::pointers::LinkedList::Cons;
    use crate::learning::pointers::{create_linked_list, deref_test, first_box, ref_test};

    #[test]
    fn it_should_work() {
        println!("{:?}", create_linked_list());
        ref_test();
        deref_test();
    }
}
