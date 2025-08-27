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

    pub async fn get_track(&self) -> Vec<TrackHeader> {
        let time = TimeData::get_time();
        let season = TimeData::get_season();
        let weather = WeatherData::get_weather().await;

        let mut filtered_tracks = Vec::new();

        filtered_tracks.append(&mut self.get_tracks_by_time(time).await);
        filtered_tracks.append(&mut self.get_tracks_by_season(season).await);
        filtered_tracks.append(&mut self.get_tracks_by_weather(weather).await);

        Self::get_most_matched_from(&filtered_tracks)
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

    fn get_most_matched_from(slice: &[TrackHeader]) -> Vec<TrackHeader> {
        let mut counts = std::collections::HashMap::new();
        for track in slice {
            *counts.entry(track.clone()).or_insert(0) += 1;
        }

        let mut sorted_tracks: Vec<_> = counts.into_iter().collect();
        sorted_tracks.sort_by(|a, b| b.1.cmp(&a.1));

        sorted_tracks.into_iter().map(|(track, _)| track).collect()
    }

    
}
