use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // The first Cons value holds 1 and another List value. 
    // This List value is another Cons value that holds 2 and another List value. 
    // This List value is one more Cons value that holds 3 and a List value, 
    // which is finally Nil, the non-recursive variant that signals the end of the list.
    // let List = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    // We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly. 
    //So, if we need to force a value to be cleaned up early, we can use the std::mem::drop function.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let value = Rc::new(RefCell::new(5));

    // Rc::clone does not make a deep copy of all data. It only increments the reference count
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let b1 = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", b1);
}
