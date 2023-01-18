use std::vec::Vec;

#[derive(Default, Debug)]
pub struct MaxHeap<T: Ord> {
	data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
	pub fn new() -> Self {
		MaxHeap { data: Vec::new() }
	}

	pub fn new_with_data(data: Vec<T>) -> Self {
		let mut ret = MaxHeap { data };
		let mut i = (ret.data.len() - 1) / 2;
		loop {
			ret.down_heap(i);
			if i == 0 {
				break;
			}
			i = i - 1;
		}
		ret
	}

	pub fn insert(&mut self, v: T) {
		self.data.push(v);
		self.up_heap();
	}

	pub fn extract(&mut self) -> Option<T> {
		match self.data.len() {
			0 => None,
			1 => self.data.pop(),
			len => {
				self.data.swap(0, len - 1);
				let ret_val = self.data.pop();
				self.down_heap(0);
				ret_val
			}
		}
	}

	fn up_heap(&mut self) {
		let mut i = self.data.len() - 1;
		while i > 0 {
			let elem = &self.data[i];
			let parent_i = (i - 1) / 2;
			let parent = &self.data[parent_i];

			if *parent >= *elem {
				break;
			}

			self.data.swap(i, parent_i);
			i = parent_i;
		}
	}

	fn down_heap(&mut self, i: usize) {
		let left = i * 2 + 1;
		let right = i * 2 + 2;
		let mut largest = i;
		let len = self.data.len();

		if left < len && self.data[left] > self.data[largest] {
			largest = left;
		}
		if right < len && self.data[right] > self.data[largest] {
			largest = right;
		}

		if i != largest {
			self.data.swap(i, largest);
			self.down_heap(largest);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_heap() {
		let mut heap = MaxHeap::new();
		heap.insert(9);
		heap.insert(20);
		heap.insert(3);
		heap.insert(4);
		heap.insert(40);
		heap.insert(0);
		heap.insert(97);

		assert_eq!(heap.extract(), Some(97));
		assert_eq!(heap.extract(), Some(40));
		assert_eq!(heap.extract(), Some(20));
		assert_eq!(heap.extract(), Some(9));
		assert_eq!(heap.extract(), Some(4));
		assert_eq!(heap.extract(), Some(3));
		assert_eq!(heap.extract(), Some(0));
		assert_eq!(heap.extract(), None);
	}

	#[test]
	fn test_heap_with_data() {
		let mut heap = MaxHeap::new_with_data(vec![9, 20, 3, 4, 40, 0, 97]);

		assert_eq!(heap.extract(), Some(97));
		assert_eq!(heap.extract(), Some(40));
		assert_eq!(heap.extract(), Some(20));
		assert_eq!(heap.extract(), Some(9));
		assert_eq!(heap.extract(), Some(4));
		assert_eq!(heap.extract(), Some(3));
		assert_eq!(heap.extract(), Some(0));
		assert_eq!(heap.extract(), None);

		let mut heap = MaxHeap::new_with_data(vec![7, 9, 2, 15, 10, 5, 12]);

		assert_eq!(heap.extract(), Some(15));
		assert_eq!(heap.extract(), Some(12));
		assert_eq!(heap.extract(), Some(10));
		assert_eq!(heap.extract(), Some(9));
		assert_eq!(heap.extract(), Some(7));
		assert_eq!(heap.extract(), Some(5));
		assert_eq!(heap.extract(), Some(2));
		assert_eq!(heap.extract(), None);
	}
}
