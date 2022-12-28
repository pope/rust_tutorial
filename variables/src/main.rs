// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
// 	let mut x = 5;
// 	println!("The value of x is: {x}");
// 	x = 6;
// 	println!("The value of x is: {x}");
// }

// fn main() {
// 	let x = 5;
//
// 	let x = x + 1;
//
// 	{
// 		let x = x * 2;
// 		println!("The value of x in the inner scope is: {x}");
// 	}
//
// 	println!("The value of x is: {x}");
// }

// fn main() {
// 	let sum = 5 + 10;
// 	println!("Sum: {sum}");
//
// 	let difference = 95.5 - 4.3;
// 	println!("Difference: {difference}");
//
// 	let product = 4 * 30;
// 	println!("Product: {product}");
//
// 	let quotient = 56.7 / 32.2;
// 	println!("Quotient: {quotient}");
// 	let truncated = -5 / 3;
// 	println!("Truncated {truncated}");
//
// 	let remainder = 43 % 5;
// 	println!("Remainer: {remainder}");
// }

// fn main() {
// 	let c = 'z';
// 	let z: char = 'ℤ';
// 	let heart_eyed_cat = '😻';
//
// 	println!("{c} - {z} - {heart_eyed_cat}");
// }

// fn main() {
// 	let y = {
// 		let x = 3;
// 		x + 1 // notice, no semicolon.
// 	};
//
// 	println!("The value of y is: {y}");
//
// 	let x = five();
// 	println!("Five {x}");
// }
//
// fn five() -> i32 {
// 	5
// }

// fn main() {
// 	let mut count = 0;
//
// 	'counting_up: loop {
// 		println!("count = {count}");
//
// 		let mut remaining = 10;
//
// 		loop {
// 			println!("remaining = {remaining}");
// 			if remaining == 9 {
// 				break;
// 			}
// 			if count == 2 {
// 				break 'counting_up;
// 			}
// 			remaining -= 1;
// 		}
//
// 		count += 1;
// 	}
//
// 	println!("End count = {count}");
// }

fn main() {
	for number in (1..4).rev() {
		println!("{number}!");
	}
	println!("LIFTOFF!!!");
}
