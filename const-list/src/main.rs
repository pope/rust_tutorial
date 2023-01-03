use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
	Cons(i32, Box<List>),
	Nil,
}

#[derive(Debug)]
enum RcList {
	RcCons(i32, Rc<RcList>),
	RcNil,
}

#[derive(Debug)]
enum RcRefList {
	RcRefCons(Rc<RefCell<i32>>, Rc<RcRefList>),
	RcRefNil,
}

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RcRefList::{RcRefCons, RcRefNil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> Self {
		Self(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn hello(name: &str) {
	println!("Hello, {name}!");
}

#[derive(Debug)]
struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data `{}`!", self.data);
	}
}

fn main() {
	let list = Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))));
	println!("{:?}", list);

	let x = 5;
	// let y = &x;
	let y = MyBox::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	let m = MyBox::new(String::from("Rust"));
	hello(&m);

	let c = CustomSmartPointer {
		data: String::from("c"),
	};
	let d = CustomSmartPointer {
		data: String::from("d"),
	};
	let e = CustomSmartPointer {
		data: String::from("e"),
	};
	println!("CustomSmartPointers created. {:?}, {:?}, {:?}", &c, &d, &e);

	drop(d);
	println!("CustomSmartPointer dropped before end of main");

	let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
	println!("count after creating a = {}", Rc::strong_count(&a));
	let _b = RcCons(3, Rc::clone(&a));
	println!("count after creating a = {}", Rc::strong_count(&a));
	{
		let _c = RcCons(4, Rc::clone(&a));
		println!("count after creating c = {}", Rc::strong_count(&a));
	}
	println!("count after c goes out of scope = {}", Rc::strong_count(&a));

	let value = Rc::new(RefCell::new(5));

	let a = Rc::new(RcRefCons(Rc::clone(&value), Rc::new(RcRefNil)));

	let b = RcRefCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
	let c = RcRefCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

	*value.borrow_mut() += 10;

	println!("a after = {:?}", a);
	println!("b after = {:?}", b);
	println!("c after = {:?}", c);
}
