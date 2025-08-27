use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{data_collector::{Season, TimeData, TimePeriod, Weather, WeatherData}, database::{Mp3Database, TrackHeader}};

pub struct Recommender {
    database: Arc<RwLock<Mp3Database>>,
}

impl Recommender {
    pub fn new(database: Arc<RwLock<Mp3Database>>) -> Self {
        Self { database }
    }

    pub async fn get_track(&self) -> Option<TrackHeader> {
        let time = TimeData::get_time();
        let season = TimeData::get_season();
        let weather = WeatherData::get_weather().await;

        let mut filtered_tracks = Vec::new();

        filtered_tracks.append(&mut self.get_tracks_by_time(time).await);
        filtered_tracks.append(&mut self.get_tracks_by_season(season).await);
        filtered_tracks.append(&mut self.get_tracks_by_weather(weather).await);

        let result = get_most_matched_from(&filtered_tracks);
        if let Some(track) = result {
            Some(track.clone())
        } else {
            None
        }
    }

    async fn get_tracks_by_time(&self, time: TimePeriod) -> Vec<TrackHeader> {
        let time_str = match time {
            TimePeriod::Dawn(_) => "dawn",
            TimePeriod::Morning(_) => "morning",
            TimePeriod::Noon(_) => "noon",
            TimePeriod::Afternoon(_) => "afternoon",
            TimePeriod::Evening(_) => "evening",
            TimePeriod::Dusk(_) => "dusk",
            TimePeriod::Night(_) => "night",
        };

        self.database
            .read().await
            .get_tracks_by_vibes(&[time_str]).await
            .unwrap_or(Vec::new())
    }

    async fn get_tracks_by_season(&self, season: Season) -> Vec<TrackHeader> {
        let season_str = match season {
            Season::Spring(_) => "spring",
            Season::Summer(_) => "summer",
            Season::Autumn(_) => "autumn",
            Season::Winter(_) => "winter",
        };

        self.database
            .read().await
            .get_tracks_by_vibes(&[season_str]).await
            .unwrap_or(Vec::new())
    }

    async fn get_tracks_by_weather(&self, weather: Weather) -> Vec<TrackHeader> {
        let weather_str = match weather {
            Weather::Sunny(_) => "sunny",
            Weather::Cloudy(_) => "cloudy",
            Weather::Rainy(_) => "rainy",
            Weather::Stormy(_) => "stormy",
            Weather::Windy(_) => "windy",
            Weather::Coldy(_) => "coldy",
            Weather::Hotty(_) => "hotty",
            Weather::Default(_) => "none",
        };

        self.database
            .read().await
            .get_tracks_by_vibes(&[weather_str]).await
            .unwrap_or(Vec::new())
    }

    
}

pub fn get_most_matched_from<T: PartialEq>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }

    let mut most_frequent_item = &slice[0];
    let mut max_count = 0;

    for i in 0..slice.len() {
        let mut current_count = 0;
        for j in 0..slice.len() {
            if slice[i] == slice[j] {
                current_count += 1;
            }
        }

        if current_count > max_count {
            max_count = current_count;
            most_frequent_item = &slice[i];
        }
    }

    Some(most_frequent_item)
}