use simplest_lockfree_hashtable::HashTable2;
use std::sync::Arc;
use std::thread;

// AI generated
fn main() {
    // 1. Initialize the table.
    // Size must be a power of 2 (checked by assertion in lib.rs).
    // We wrap it in Arc to share ownership across threads.
    let table = Arc::new(HashTable2::new(32));

    // 2. Spawn a thread to write even keys
    let t_even = {
        let table = table.clone();
        thread::spawn(move || {
            for i in 1..=10 {
                let key = i * 2;
                let val = i * 200;
                println!("Thread 1 inserting: Key {} -> Val {}", key, val);
                table.set_item(key, val);
            }
        })
    };

    // 3. Spawn a thread to write odd keys
    let t_odd = {
        let table = table.clone();
        thread::spawn(move || {
            for i in 1..=10 {
                let key = (i * 2) + 1;
                let val = i * 100;
                println!("Thread 2 inserting: Key {} -> Val {}", key, val);
                table.set_item(key, val);
            }
        })
    };

    // 4. Wait for threads to finish
    t_even.join().expect("Thread even panicked");
    t_odd.join().expect("Thread odd panicked");

    // 5. Verify results on the main thread
    println!("\n--- Reading Results ---");
    let val_even = table.get_item(4); // 2 * 2
    let val_odd = table.get_item(5); // (2 * 2) + 1

    println!("Key 4: {}", val_even);
    println!("Key 5: {}", val_odd);

    // Assertions to ensure correctness
    assert_eq!(val_even, 400); // 2 * 200
    assert_eq!(val_odd, 200); // 2 * 100
}
