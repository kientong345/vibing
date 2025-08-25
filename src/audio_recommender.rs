use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{audio_services::Audio, data_collector::{TimeData, WeatherData}, database::Mp3Database};


pub struct Recommender {
    database: Arc<RwLock<Mp3Database>>,
}

impl Recommender {
    pub fn new(database: Arc<RwLock<Mp3Database>>) -> Self {
        Self { database }
    }

    pub async fn get_audio() -> Audio {
        let time = TimeData::get_time();
        let season = TimeData::get_season();
        let weather = WeatherData::get_weather().await;


        todo!()
    }
}