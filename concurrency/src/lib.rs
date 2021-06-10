#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn mov() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("vector: {:?}", v)
        });

        handle.join().unwrap();
    }

    #[test]
    fn channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(String::from("hi")).unwrap();
        });

        let rev = rx.recv().unwrap();

        println!("got: {}", rev)
    }

    #[test]
    fn mutex() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap(); // num is MutexGuard<i32>
            *num = 6;
        } // release lock automatically by Drop implementation

        println!("m = {:?}", m);
    }

    #[test]
    fn arc() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
