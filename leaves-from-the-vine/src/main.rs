use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
	_value: i32,
	parent: RefCell<Weak<Node>>,
	_children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
	println!("Part 1");
	println!("----------------------------------------");
	run1();

	println!();
	println!("Part 2");
	println!("----------------------------------------");
	run2();
}

fn run1() {
	let leaf = Rc::new(Node {
		_value: 3,
		parent: RefCell::new(Weak::new()),
		_children: RefCell::new(vec![]),
	});

	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

	let branch = Rc::new(Node {
		_value: 5,
		parent: RefCell::new(Weak::new()),
		_children: RefCell::new(vec![Rc::clone(&leaf)]),
	});
	*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn run2() {
	let leaf = Rc::new(Node {
		_value: 3,
		parent: RefCell::new(Weak::new()),
		_children: RefCell::new(vec![]),
	});

	println!(
		"leaf strong = {}, weak = {}",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);

	{
		let branch = Rc::new(Node {
			_value: 5,
			parent: RefCell::new(Weak::new()),
			_children: RefCell::new(vec![Rc::clone(&leaf)]),
		});
		*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

		println!(
			"branch strong = {}, weak = {}",
			Rc::strong_count(&branch),
			Rc::weak_count(&branch)
		);
		println!(
			"leaf strong = {}, weak = {}",
			Rc::strong_count(&leaf),
			Rc::weak_count(&leaf)
		);
	}

	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
	println!(
		"leaf strong = {}, weak = {}",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);
}
