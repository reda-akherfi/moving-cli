use tokio::time::{sleep, Duration};
use chrono::Local;

pub async fn start_timer(work: u64, rest: u64, rounds: u32) {
    for round in 1..=rounds {
        println!("Round {}: Work for {} seconds", round, work);
        let start_time = Local::now();
        println!("Start Time: {}", start_time.format("%H:%M:%S").to_string());

        sleep(Duration::from_secs(work)).await;

        let end_time = Local::now();
        println!("End Time: {}", end_time.format("%H:%M:%S").to_string());

        if round < rounds {
            println!("Rest for {} seconds", rest);
            sleep(Duration::from_secs(rest)).await;
        }
    }
    println!("HIIT session complete!");
}

