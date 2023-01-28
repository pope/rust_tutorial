#[derive(Debug)]
struct Rect {
	width: u32,
	height: u32,
}

impl Rect {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}

	fn has_width(&self) -> bool {
		self.width > 0
	}

	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rect) -> bool {
		self.width > other.width && self.height > other.height
	}
}

fn main() {
	let rect = Rect {
		width: 30,
		height: 50,
	};

	println!("rect is {rect:#?}");
	dbg!(&rect);
	println!("Does the rectangle have a width?: {}", rect.has_width());
	println!(
		"The area of the rectangle is {} square pixels.",
		rect.area()
	);

	let smaller_rect = Rect::square(20);
	println!(
		"Can rect hold smaller_rect? {}",
		rect.can_hold(&smaller_rect)
	);

	let longer_rect = Rect {
		width: 20,
		height: 60,
	};
	println!("Can rect hold longer_rect? {}", rect.can_hold(&longer_rect));
}

//// PHASE 3
// #[derive(Debug)]
// struct Rect {
// 	width: u32,
// 	height: u32,
// }
//
// fn main() {
// 	let rect = Rect {
// 		width: 30,
// 		height: 50,
// 	};
//
// 	println!("rect is {:#?}", rect);
// 	dbg!(&rect);
// 	println!(
// 		"The area of the rectangle is {} square pixels.",
// 		area(&rect)
// 	);
// }
//
// fn area(rect: &Rect) -> u32 {
// 	rect.width * rect.height
// }

//// PHASE 2
// fn main() {
// 	let dimensions = (30, 50);
//
// 	println!(
// 		"The area of the rectangle is {} square pixels.",
// 		area(dimensions)
// 	);
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
// 	dimensions.0 * dimensions.1
// }

//// PHASE 1
// fn main() {
// 	let width = 30;
// 	let height = 50;
//
// 	println!(
// 		"The area of the rectangle is {} square pixels.",
// 		area(width, height)
// 	);
// }
//
// fn area(width: u32, height: u32) -> u32 {
// 	width * height
// }
