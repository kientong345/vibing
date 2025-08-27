use std::{sync::Arc, time::Duration};

use tokio::{sync::RwLock, time::sleep};
use vibing::{audio_recommender::Recommender, audio_services::Audio, data_collector::{TimeData, WeatherData}, database::Mp3Database};

#[tokio::main]
async fn main() {
    // let database_url = std::env::var("DATABASE_URL")
    //         .expect("DATABASE_URL is not existed in .env");

    let database = Arc::new(
        RwLock::new(
            Mp3Database::new("sqlite://vibing_library.sqlite").await.expect("db error")
        )
    );

    let recommender = Arc::new(
        RwLock::new(
            Recommender::new(database)
        )
    );

    let weather = WeatherData::get_weather().await;
    println!("{:?}", weather);

    let time = TimeData::get_time();
    println!("{:?}", time);

    let season = TimeData::get_season();
    println!("{:?}", season);

    let tracks = recommender.read().await.get_track().await;

    for track in tracks {
        println!("now play: {:?} - {:?}", track.path, track.vibes);
        let mut audio = Audio::new(&track.path);
        audio.set_volume(0.2);
        audio.play();

        while !audio.is_end() {
            sleep(Duration::from_secs(1)).await;
        }
    }

}