use std::cmp;
use std::cmp::Reverse;
use std::convert::TryFrom;

use binary_heap::MaxHeap;

#[derive(Debug, PartialEq)]
struct Point {
	distance: f64,
	point: Vec<i32>,
}

impl PartialOrd for Point {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.distance.partial_cmp(&other.distance)
	}
}

impl Ord for Point {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.distance.total_cmp(&other.distance)
	}
}

impl Eq for Point {}

pub fn k_nearest(points: Vec<Vec<i32>>, k: u32) -> Vec<Vec<i32>> {
	let k_size = usize::try_from(k).unwrap_or(usize::MAX);
	let cap = cmp::min(k_size, points.len());

	let mut heap = MaxHeap::new();
	for point in points {
		let distance: i32 = point.iter().map(|p| p.pow(2)).sum();
		let distance: f64 = (distance as f64).sqrt();
		heap.insert(Reverse(Point { distance, point }));
	}
	let mut results = Vec::with_capacity(cap);
	for _ in 0..cap {
		results.push(heap.extract().unwrap().0.point);
	}
	// Keeping this because I think the `Some(Reverse(v))` part is cool.
	// for _ in 0..cap {
	// 	if let Some(Reverse(v)) = heap.extract() {
	// 		results.push(v.point);
	// }
	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works_with_empty_points() {
		let result = k_nearest(Vec::new(), 3);
		assert_eq!(result, Vec::<Vec<i32>>::new());
	}

	#[test]
	fn it_works_with_one_point() {
		let result = k_nearest(vec![vec![0, 0]], 3);
		assert_eq!(result, vec![vec![0, 0]]);
	}

	#[test]
	fn it_works_top_k_points() {
		let result = k_nearest(vec![vec![3, 9], vec![2, 2], vec![1, 1]], 2);
		assert_eq!(result, vec![vec![1, 1], vec![2, 2]]);
	}
}
