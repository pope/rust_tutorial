use std::cmp::Reverse;

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

pub fn k_nearest(points: Vec<Vec<i32>>, k: usize) -> Vec<Vec<i32>> {
	points
		.into_iter()
		.map(|point| {
			let distance: i32 = point.iter().map(|p| p.pow(2)).sum();
			let distance: f64 = (distance as f64).sqrt();
			Reverse(Point { distance, point })
		})
		.collect::<MaxHeap<_>>()
		.into_iter()
		.take(k)
		.map(|Reverse(p)| p.point)
		.collect()
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
