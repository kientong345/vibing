use std::time::Duration;

use tokio::time::sleep;
use vibing::{audio_services::Audio, data_collector::WeatherData};



#[tokio::main]
async fn main() {
    let weather = WeatherData::get_weather().await;

    println!("{:?}", weather);

    let mut audio = Audio::new("/home/kt345/Downloads/793250_Raw-Unfiltered-Calamity.mp3");

    audio.set_volume(0.2);

    audio.play();

    sleep(Duration::from_millis(20000)).await;

    audio.pause();

    sleep(Duration::from_millis(3000)).await;

    audio.play();

    sleep(Duration::from_millis(110000)).await;

}