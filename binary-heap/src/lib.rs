#[derive(Default, Debug)]
pub struct MaxHeap<T: Ord> {
	data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
	pub fn new() -> Self {
		MaxHeap { data: Vec::new() }
	}

	pub fn push(&mut self, v: T) {
		self.data.push(v);
		self.up_heap();
	}

	pub fn pop(&mut self) -> Option<T> {
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

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
	fn from(data: Vec<T>) -> Self {
		let mut ret = MaxHeap { data };

		if ret.data.is_empty() {
			return ret;
		}

		let mut i = (ret.data.len() - 1) / 2;
		loop {
			ret.down_heap(i);
			if i == 0 {
				break;
			}
			i -= 1;
		}
		ret
	}
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		MaxHeap::from(iter.into_iter().collect::<Vec<T>>())
	}
}

impl<T: Ord> IntoIterator for MaxHeap<T> {
	type Item = T;
	type IntoIter = MaxOrderedIterator<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		MaxOrderedIterator { heap: self }
	}
}

pub struct MaxOrderedIterator<T: Ord> {
	heap: MaxHeap<T>,
}

impl<T: Ord> Iterator for MaxOrderedIterator<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.heap.pop()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_heap() {
		let mut heap = MaxHeap::new();
		heap.push(9);
		heap.push(20);
		heap.push(3);
		heap.push(4);
		heap.push(40);
		heap.push(0);
		heap.push(97);

		assert_eq!(heap.pop(), Some(97));
		assert_eq!(heap.pop(), Some(40));
		assert_eq!(heap.pop(), Some(20));
		assert_eq!(heap.pop(), Some(9));
		assert_eq!(heap.pop(), Some(4));
		assert_eq!(heap.pop(), Some(3));
		assert_eq!(heap.pop(), Some(0));
		assert_eq!(heap.pop(), None);
	}

	#[test]
	fn test_heap_with_data() {
		let heap = MaxHeap::from(vec![9, 20, 3, 4, 40, 0, 97]);

		assert_eq!(
			heap.into_iter().collect::<Vec<i32>>(),
			vec![97, 40, 20, 9, 4, 3, 0]
		);

		let heap = MaxHeap::from(vec![7, 9, 2, 15, 10, 5, 12]);

		assert_eq!(
			heap.into_iter().collect::<Vec<i32>>(),
			vec![15, 12, 10, 9, 7, 5, 2]
		);
	}

	#[test]
	fn test_heap_with_empty_vec() {
		let heap: MaxHeap<i32> = vec![].into();

		assert_eq!(heap.into_iter().collect::<Vec<i32>>(), vec![]);
	}

	#[test]
	fn test_heap_with_one_item() {
		let heap: MaxHeap<_> = vec![32].into();

		assert_eq!(heap.into_iter().collect::<Vec<i32>>(), vec![32]);
	}
}
