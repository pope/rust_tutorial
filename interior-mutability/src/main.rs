// Following along from https://www.youtube.com/watch?v=HwupNf9iCJk

use std::cell::{Cell, RefCell};
use std::iter;
use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Debug)]
struct IntNode<'a> {
	value: Cell<i32>,
	adjacent: Vec<&'a IntNode<'a>>,
}

impl<'a> IntNode<'a> {
	fn add_one(&self) {
		let new_value = self.value.get() + 1;
		self.value.set(new_value);

		self.adjacent.iter().for_each(|n| n.add_one());
	}
}

#[derive(Debug)]
struct StringNode<'a> {
	value: RefCell<String>,
	adjacent: Vec<&'a StringNode<'a>>,
}

impl<'a> StringNode<'a> {
	fn add_urgency(&self) {
		iter::once(&self).chain(self.adjacent.iter()).for_each(|n| {
			n.value.borrow_mut().push('!');
		});
	}
}

#[derive(Debug)]
struct ThreadSafeNode {
	value: RwLock<String>,
	adjacent: Vec<Arc<ThreadSafeNode>>,
}

impl ThreadSafeNode {
	fn add_urgency(&self) {
		{
			let mut value = self.value.write().unwrap();
			value.push('!');
		}
		self.adjacent.iter().for_each(|n| n.add_urgency());
	}
}

fn main() {
	let a = IntNode {
		value: Cell::new(1),
		adjacent: vec![],
	};

	let b = IntNode {
		value: Cell::new(2),
		adjacent: vec![&a],
	};

	let c = IntNode {
		value: Cell::new(3),
		adjacent: vec![&a],
	};

	a.add_one();
	c.add_one();

	dbg!(&a);
	dbg!(&b);
	dbg!(&c);

	let x = StringNode {
		value: "X".to_owned().into(),
		adjacent: vec![],
	};
	let y = StringNode {
		value: "Y".to_owned().into(),
		adjacent: vec![&x],
	};
	let z = StringNode {
		value: "Z".to_owned().into(),
		adjacent: vec![&x, &y],
	};

	y.add_urgency();

	dbg!(&z);

	let x = Arc::new(ThreadSafeNode {
		value: "X".to_owned().into(),
		adjacent: vec![],
	});
	let y = Arc::new(ThreadSafeNode {
		value: "Y".to_owned().into(),
		adjacent: vec![x.clone()],
	});
	let z = Arc::new(ThreadSafeNode {
		value: "Z".to_owned().into(),
		adjacent: vec![x.clone(), y.clone()],
	});

	let t1 = thread::spawn(move || {
		x.add_urgency();
	});
	let t2 = thread::spawn(move || {
		y.add_urgency();
	});
	t1.join().unwrap();
	t2.join().unwrap();

	dbg!(z);
}
