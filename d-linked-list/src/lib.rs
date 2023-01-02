use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T: Copy + fmt::Debug> {
	pub value: T,
	pub next: Option<NodePtr<T>>,
	pub prev: Option<NodeWeakPtr<T>>,
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;
type NodeWeakPtr<T> = Weak<RefCell<Node<T>>>;

impl<T: Copy + fmt::Debug> Node<T> {
	pub fn new(value: T) -> Self {
		Node {
			value,
			next: None,
			prev: None,
		}
	}
}

impl<T: Copy + fmt::Debug> From<Node<T>> for Option<NodePtr<T>> {
	fn from(node: Node<T>) -> Self {
		Some(Rc::new(RefCell::new(node)))
	}
}

#[derive(Debug)]
pub struct List<T: Copy + fmt::Debug> {
	head: Option<NodePtr<T>>,
	tail: Option<NodePtr<T>>,
}

impl<T: Copy + fmt::Debug> List<T> {
	pub fn new() -> Self {
		List {
			head: None,
			tail: None,
		}
	}

	pub fn push_back(&mut self, value: T) {
		let mut node = Node::new(value);

		match &mut self.tail.take() {
			None => {
				debug_assert!(
					self.head.is_none(),
					"If tail is None, head must be None"
				);
				self.head = node.into();
				self.tail = self.head.clone();
			}
			Some(current_tail) => {
				debug_assert!(
					current_tail.borrow().next.is_none(),
					"Tail should not have a next entry"
				);
				node.prev = Some(Rc::downgrade(current_tail));
				self.tail = node.into();
				current_tail.borrow_mut().next = self.tail.clone();
			}
		}
	}

	pub fn pop_back(&mut self) -> Option<T> {
		match &mut self.tail.take() {
			None => {
				debug_assert!(
					self.head.is_none(),
					"If tail is None, head must be None"
				);
				None
			}
			Some(tail_ptr) => {
				let mut tail = tail_ptr.borrow_mut();
				debug_assert!(
					tail.next.is_none(),
					"Tail should not have a next entry"
				);
				match tail.prev.take() {
					None => {
						debug_assert!(
							Rc::ptr_eq(
								self.head
									.as_ref()
									.expect("Head should have a value"),
								tail_ptr
							),
							"When the tail's previous value is None, then the \
								tail and head values should be the same \
								instance."
						);
						self.head.take();
					}
					Some(prev) => {
						match prev.upgrade() {
							Some(prev) => {
								prev.borrow_mut().next = None;
								self.tail = Some(prev);
							}
							None => {
								debug_assert!(
									self.head.is_none(),
									"If the previous value is None, then head \
										shouldn't have a value either"
								);
							}
						};
					}
				};
				Some(tail.value)
			}
		}
	}

	pub fn push_front(&mut self, value: T) {
		let mut node = Node::new(value);

		match &mut self.head.take() {
			None => {
				debug_assert!(
					self.tail.is_none(),
					"If head is None, tail must be None"
				);
				self.head = node.into();
				self.tail = self.head.clone();
			}
			Some(current_head) => {
				debug_assert!(
					current_head.borrow().prev.is_none(),
					"Head should not have a previous entry"
				);
				node.next = Some(current_head.clone());
				self.head = node.into();
				current_head.borrow_mut().prev =
					Some(Rc::downgrade(self.head.as_ref().unwrap()));
			}
		};
	}

	pub fn pop_front(&mut self) -> Option<T> {
		match &mut self.head.take() {
			None => {
				debug_assert!(
					self.tail.is_none(),
					"If head is None, tail should be None"
				);
				None
			}
			Some(head_ptr) => {
				let mut head = head_ptr.borrow_mut();
				debug_assert!(
					head.prev.is_none(),
					"Head should not have a previous entry"
				);
				match head.next.take() {
					None => {
						debug_assert!(
							Rc::ptr_eq(
								self.tail
									.as_ref()
									.expect("Tail should have a value"),
								head_ptr
							),
							"When the head's next value is None, then the \
								tail and head values should be the same \
								instance."
						);
						self.tail.take();
					}
					Some(next) => {
						debug_assert!(
							next.borrow()
								.prev
								.as_ref()
								.expect(
									"The new next should have a previous entry"
								)
								.upgrade()
								.is_some(),
							"There should be a reference to the previous entry"
						);
						next.borrow_mut().prev = None;
						self.head = Some(next);
					}
				};
				Some(head.value)
			}
		}
	}

	pub fn iter(&self) -> ListIterator<T> {
		ListIterator {
			current: self.head.clone(),
			current_back: self.tail.clone(),
		}
	}
}

impl<T: Copy + fmt::Debug> Default for List<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Copy + fmt::Debug> Drop for List<T> {
	fn drop(&mut self) {
		while self.pop_back().is_some() {}
	}
}

pub struct ListIterator<T: Copy + fmt::Debug> {
	current: Option<NodePtr<T>>,
	current_back: Option<NodePtr<T>>,
}

impl<T: Copy + fmt::Debug> Iterator for ListIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.current.take() {
			None => None,
			Some(current) => {
				let current = current.borrow();
				self.current = current.next.clone();
				Some(current.value)
			}
		}
	}
}

impl<T: Copy + fmt::Debug> DoubleEndedIterator for ListIterator<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		match self.current_back.take() {
			None => None,
			Some(current) => {
				let current = current.borrow();
				if let Some(prev) = &current.prev {
					self.current_back = prev.upgrade();
				}
				Some(current.value)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn manages_list_from_back() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);

		assert_eq!(list.pop_back(), Some(4));
		assert_eq!(list.pop_back(), Some(3));
		assert_eq!(list.pop_back(), Some(2));
		assert_eq!(list.pop_back(), Some(1));
		assert_eq!(list.pop_back(), None);
	}

	#[test]
	fn manages_list_from_front() {
		let mut list = List::new();
		list.push_front(4);
		list.push_front(3);
		list.push_front(2);
		list.push_front(1);

		assert_eq!(list.pop_front(), Some(1));
		assert_eq!(list.pop_front(), Some(2));
		assert_eq!(list.pop_front(), Some(3));
		assert_eq!(list.pop_front(), Some(4));
		assert_eq!(list.pop_front(), None);
	}

	#[test]
	fn iterates_through_list() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);

		// Or

		let expected = vec![1, 2, 3, 4];
		assert_eq!(list.iter().collect::<Vec<i32>>(), expected);
	}

	#[test]
	fn iterates_through_list_in_reverse() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);

		assert_eq!(list.iter().rev().collect::<Vec<i32>>(), vec![4, 3, 2, 1]);
	}
}
