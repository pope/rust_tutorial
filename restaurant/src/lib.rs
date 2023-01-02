mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	// All the same. Just different ways to access the method.
	crate::front_of_house::hosting::add_to_waitlist();
	front_of_house::hosting::add_to_waitlist();
	hosting::add_to_waitlist();

	let _order1 = back_of_house::Appetizer::Soup;
	let _order2 = back_of_house::Appetizer::Salad;
}

fn _deliver_order() {}

mod back_of_house {
	pub struct _Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl _Breakfast {
		pub fn _summer(toast: &str) -> _Breakfast {
			_Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}

	fn _fix_incorrect_order() {
		_cook_order();
		super::_deliver_order();
		crate::_deliver_order();
	}

	fn _cook_order() {}
}

mod customer {
	use crate::front_of_house::hosting;

	pub fn _eat_at_restaurant() {
		hosting::add_to_waitlist();
	}
}
