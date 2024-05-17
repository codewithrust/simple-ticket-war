use std::sync::{Arc, Mutex};
use std::thread;

fn buy_ticket(
    buyer_id: i32,
    num_available_tickets: Arc<Mutex<i32>>,
    num_sold_tickets: Arc<Mutex<i32>>,
) {
    if *num_available_tickets.lock().unwrap() <= 0 {
        println!("Buyer {} missed the ticket.", buyer_id);
        return;
    }
    // We need to lock the Mutex to access the data
    // We need to unwrap the result of the lock method to get the value
    // We need to use the dereference operator (*) to access the value
    *num_available_tickets.lock().unwrap() -= 1;
    *num_sold_tickets.lock().unwrap() += 1;
    println!(
        "Buyer {} got the ticket.!. Remaining Available Tickets: {}",
        buyer_id,
        num_available_tickets.lock().unwrap()
    );
}

fn main() {
    // Arc is a thread-safe reference-counted pointer
    // Mutex is a mutual exclusion primitive useful for protecting shared data
    // We can use Arc<Mutex<T>> to share a mutable value between threads
    let num_available_tickets = Arc::new(Mutex::new(20));
    let num_sold_tickets = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for index in 0..30 {
        // We need to clone the Arc to pass it to the thread
        // We can clone the Arc because it only increments the reference count
        // and does not create a new instance of the data
        let num_available_tickets = Arc::clone(&num_available_tickets);
        let num_sold_tickets = Arc::clone(&num_sold_tickets);

        // We need to move the Arc to the thread
        // We can move the Arc because it only increments the reference count
        let handle =
            thread::spawn(move || buy_ticket(index, num_available_tickets, num_sold_tickets));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Total Available Tickets: {}",
        num_available_tickets.lock().unwrap()
    );

    println!("Total Sold Tickets: {}", num_sold_tickets.lock().unwrap());
}
