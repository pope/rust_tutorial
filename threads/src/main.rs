use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
	simple_run();
	thread_with_move();
	using_channels();
	multiple_sends();
	multiple_sends_with_multiple_senders();
	mutex_counting();
	mutex_counting2();
}

fn simple_run() {
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("hi number {i} from the spawned thread!");
			thread::sleep(Duration::from_millis(1));
		}
	});
	for i in 1..10 {
		println!("hi number {i} from the main thread!");
		thread::sleep(Duration::from_millis(1));
	}

	handle.join().unwrap();
}

fn thread_with_move() {
	let v = vec![1, 2, 3];
	let handle = thread::spawn(move || {
		println!("Here's a vector: {v:?}");
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
	println!("Got [1]: {val}");
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
		println!("Got [2]: {recv}");
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
		println!("Got [3]: {recv}");
	}
}

fn mutex_counting() {
	let counter = Arc::new(Mutex::new(0));
	let mut handlers = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
		handlers.push(handle);
	}

	for handle in handlers {
		handle.join().unwrap();
	}

	println!("Mutex Count Result: {}", *counter.lock().unwrap());
}

// This does the same thing as `mutex_counting`, but using a functional
// style.
//
// What was good to note was that the `collect` is required to batch up all
// of the threads to spawn. If `collect` is removed, then the iterator will
// process each thread one at a time - and since `join` blocks, we're not
// really getting interesting parallelism then. Iterators here are like Streams
// in other languages where the values are lazily evaluated.
fn mutex_counting2() {
	let counter = Arc::new(Mutex::new(0));
	let handles: Vec<JoinHandle<()>> = (0..10)
		.map(|_| {
			let counter = Arc::clone(&counter);
			thread::spawn(move || {
				let mut num = counter.lock().unwrap();
				*num += 1;
			})
		})
		.collect();
	for handle in handles {
		handle.join().unwrap();
	}

	println!("Mutex Count Result (2): {}", *counter.lock().unwrap());
}
