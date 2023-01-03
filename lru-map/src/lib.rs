use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Weak;

use d_linked_list::{List, Node};

pub struct LRU<K, T>
where
	K: Copy + Eq + Hash,
	T: Copy,
{
	list: List<T>,
	map: HashMap<K, Weak<RefCell<Node<T>>>>,
	capacity: usize,
}

impl<K, T> LRU<K, T>
where
	K: Copy + Eq + Hash,
	T: Copy,
{
	pub fn new() -> Self {
		LRU::with_capacity(10)
	}

	pub fn with_capacity(capacity: usize) -> Self {
		LRU {
			list: List::new(),
			map: HashMap::new(),
			capacity,
		}
	}

	pub fn get(&mut self, k: K) -> Option<T> {
		self.map
			.get_mut(&k)
			.and_then(|ptr| ptr.upgrade())
			.map(|node| {
				let value = node.borrow().value;
				self.list.move_node_to_back(node);
				value
			})
	}

	pub fn put(&mut self, k: K, v: T) {
		let ptr = self.map.get_mut(&k).and_then(|p| p.upgrade());
		match ptr {
			None => {
				self.list.push_back(v);
				self.map.insert(
					k,
					self.list
						.get_weak_tail()
						.expect("Just added value, should have a tail"),
				);

				// TODO(pope): Amortize the count.
				if self.list.iter().count() > self.capacity {
					self.list.pop_front();
				}
			}
			Some(node) => {
				node.borrow_mut().value = v;
				self.list.move_node_to_back(node);
			}
		};
	}
}

impl<K, T> Default for LRU<K, T>
where
	K: Copy + Eq + Hash,
	T: Copy,
{
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builds_lru() {
		let mut lru = LRU::new();
		lru.put(1, "foo");
		lru.put(2, "bar");
		lru.put(3, "fizz");
		lru.put(4, "buzz");
		lru.put(5, "bazz");

		assert_eq!(lru.get(3), Some("fizz"));
		assert_eq!(lru.get(2), Some("bar"));
		assert_eq!(
			lru.list.iter().collect::<Vec<&str>>(),
			vec!["foo", "buzz", "bazz", "fizz", "bar"]
		);
	}

	#[test]
	fn builds_lru_with_capacity() {
		let mut lru = LRU::with_capacity(3);
		lru.put(1, "foo");
		lru.put(2, "bar");
		lru.put(3, "fizz");
		lru.put(4, "buzz");
		lru.put(5, "bazz");

		assert_eq!(lru.get(3), Some("fizz"));
		assert_eq!(lru.get(4), Some("buzz"));
		assert_eq!(
			lru.list.iter().collect::<Vec<&str>>(),
			vec!["bazz", "fizz", "buzz"]
		);
	}
}
