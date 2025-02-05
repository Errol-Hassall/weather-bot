use std::time;

use chrono::prelude::*;

use crate::weather::get_weather;

#[tokio::main]
pub async fn background_job() {
    let current_date_time = Local::now().time();
    let low = NaiveTime::from_hms_opt(5, 0, 0).unwrap();
    let high = NaiveTime::from_hms_opt(5, 1, 0).unwrap();
    if current_date_time > low && current_date_time < high {
        let _ = get_weather().await;
    }

    let sixty_seconds = time::Duration::from_millis(100 * 60);

    std::thread::sleep(sixty_seconds);
}
