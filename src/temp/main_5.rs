// mod firestore;
// mod utils;
// mod game_server;
// mod game_data;


// use actix_web::{web, App, HttpServer};


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .route("/", web::get().to(game_server::new_connection))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }




use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a shared counter protected by a Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // Clone the counter for the first thread
    let counter1 = Arc::clone(&counter);
    // Clone the counter for the second thread
    let counter2 = Arc::clone(&counter);

    // Define the number of iterations
    let iterations = 10000000;

    // Record the start time
    let start_time = Instant::now();

    // Spawn the first thread
    let handle1 = tokio::spawn(async move {
        // for _ in 0..iterations {
        //     // let mut num = counter1.lock().await;
        //     // *num += 1;
        //     //sleep(Duration::from_millis(1)).await; // Simulate some work
        // }
    });

    // Spawn the second thread
    let handle2 = tokio::spawn(async move {
        for _ in 0..iterations {
            let mut num = counter2.lock().await;
            *num += 1;
            //sleep(Duration::from_millis(1)).await; // Simulate some work
        }
    });

    // Wait for both threads to finish
    let _ = tokio::join!(handle1, handle2);

    // Record the end time
    let duration = start_time.elapsed();

    // Print the final counter value
    let final_count = *counter.lock().await;
    println!("Final counter value: {}", final_count);
    println!("Time taken: {:?}", duration);
    
}



