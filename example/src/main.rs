use std::{sync::Mutex, thread};

static COUNTER: Mutex<i32> = Mutex::new(0);

fn inc_counter() {
    let mut num = COUNTER.lock().unwrap();
    *num += 1;
}

fn main() {
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let th = thread::spawn(inc_counter);
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap();
    }

    println!("결과: {}", *COUNTER.lock().unwrap());
}
