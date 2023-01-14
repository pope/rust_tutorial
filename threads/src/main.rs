use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	simple_run();
	thread_with_move();
	using_channels();
	multiple_sends();
	multiple_sends_with_multiple_senders();
}

fn simple_run() {
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});
	for i in 1..10 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}

	handle.join().unwrap();
}

fn thread_with_move() {
	let v = vec![1, 2, 3];
	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
	});
	handle.join().unwrap();
}

fn using_channels() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let val = String::from("Message to send");
		tx.send(val).unwrap();
	});

	let val = rx.recv().unwrap();
	println!("Got [1]: {}", val);
}

fn multiple_sends() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];
		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(50));
		}
	});

	for recv in rx {
		println!("Got [2]: {}", recv);
		thread::sleep(Duration::from_millis(100));
	}
}

fn multiple_sends_with_multiple_senders() {
	let (tx, rx) = mpsc::channel();

	let tx1 = tx.clone();
	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];
		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_millis(50));
		}
	});

	thread::spawn(move || {
		let vals = vec![
			String::from("more"),
			String::from("messages"),
			String::from("for"),
			String::from("you"),
		];
		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(50));
		}
	});

	for recv in rx {
		println!("Got [3]: {}", recv);
	}
}
