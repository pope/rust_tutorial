use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
	Red,
	Blue,
}

struct Inventory {
	shirts: Vec<ShirtColor>,
}

impl Inventory {
	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked())
	}

	fn most_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;

		for color in &self.shirts {
			match color {
				ShirtColor::Blue => num_blue += 1,
				ShirtColor::Red => num_red += 1,
			};
		}

		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}
}
#[derive(Debug, Clone)]
struct Rectangle {
	width: u32,
	height: u32,
}

#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};

	let user_pref1 = Some(ShirtColor::Red);
	let giveaway1 = store.giveaway(user_pref1);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref1, giveaway1,
	);

	let user_pref2 = None;
	let giveaway2 = store.giveaway(user_pref2);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref2, giveaway2,
	);

	let list = vec![1, 2, 3];
	println!("Before defining closure: {:?}", list);

	thread::spawn(move || println!("From thread: {:?}", list))
		.join()
		.expect("Thread should complete");

	let mut list = [
		Rectangle {
			width: 10,
			height: 1,
		},
		Rectangle {
			width: 3,
			height: 5,
		},
		Rectangle {
			width: 7,
			height: 12,
		},
	];
	let mut num_sort_operations = 0;
	let mut sort_operations = vec![];

	list.sort_by_key(|r| {
		// wack, but let's me play around with cloning and scopes in here.
		sort_operations.push(r.clone());
		num_sort_operations += 1;
		r.width * r.height
	});

	println!(
		"The list: {:#?}, sorted in {} operations",
		&list, num_sort_operations
	);
	println!("The list operations: {:#?}", &sort_operations);

	let shoes = vec![
		Shoe {
			size: 10,
			style: String::from("sneaker"),
		},
		Shoe {
			size: 13,
			style: String::from("sandal"),
		},
		Shoe {
			size: 10,
			style: String::from("boot"),
		},
	];

	let in_my_size = shoes_in_size(shoes, 10);
	println!("Shoes in my size: {:#?}", in_my_size);
}
