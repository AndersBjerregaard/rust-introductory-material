pub fn arcmutexhandles() {
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = std::sync::Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("Num: {}", &num);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Could not join handle");
    }

    println!("Result: {}", *counter.lock().unwrap());
}