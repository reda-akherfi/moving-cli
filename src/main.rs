mod timer;

use tokio::time::Duration;




#[tokio::main]
async fn main() {

    // hard-coding tha values for work and rest
    let train_time: u64 = 10;
    let rest_time: u64 = 5;
    let rounds: u32 = 2;


    timer::start_timer(train_time, rest_time, rounds).await;


}

