use chrono::{self, Timelike, Datelike};
use reqwest;
use serde::Deserialize;

pub type Hour = f32;
pub type Month = u8;
pub type Temperature = f32;

#[derive(Debug, Clone, Copy)]
pub enum TimePeriod {
    Dawn(Hour),
    Morning(Hour),
    Noon(Hour),
    Afternoon(Hour),
    Dusk(Hour),
    Evening(Hour),
    Night(Hour),
}

#[derive(Debug, Clone, Copy)]
pub enum Season {
    Spring(Month),
    Summer(Month),
    Autumn(Month),
    Winter(Month),
}

#[derive(Debug, Clone, Copy)]
pub enum Weather {
    Sunny(Temperature),
    Rainy(Temperature),
    Windy(Temperature),
    Cloudy(Temperature),
    Stormy(Temperature),
    Hotty(Temperature),
    Coldy(Temperature),
    Default(Temperature),
}

#[derive(Deserialize, Debug)]
struct WeatherDesc {
    value: String,
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types, non_snake_case)]
struct current_condition {
    temp_C: String,
    weatherDesc: Vec<WeatherDesc>,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current_condition: Vec<current_condition>,
}

pub struct TimeData;

impl TimeData {
    pub fn get_time() -> TimePeriod {
        let now = chrono::Local::now();
        let hour = now.hour() as f32 + now.minute() as f32 / 60.0;

        match hour {
            h if (5.0..=7.0).contains(&h) => TimePeriod::Dawn(h),
            h if (7.0..=11.0).contains(&h) => TimePeriod::Morning(h),
            h if (11.0..=13.0).contains(&h) => TimePeriod::Noon(h),
            h if (13.0..=17.0).contains(&h) => TimePeriod::Afternoon(h),
            h if (17.0..=19.0).contains(&h) => TimePeriod::Dusk(h),
            h if (19.0..=22.0).contains(&h) => TimePeriod::Evening(h),
            _ => TimePeriod::Night(hour),
        }
    }

    pub fn get_season() -> Season {
        let now = chrono::Local::now();
        let month = now.month() as u8;
        let _day = now.day() as u8;

        match month {
            3 | 4 | 5 => Season::Spring(month),
            6 | 7 | 8 => Season::Summer(month),
            9 | 10 | 11 => Season::Autumn(month),
            _ => Season::Winter(month), // 12, 1, 2
        }
    }
}

pub struct WeatherData;

impl WeatherData {
    pub async fn get_weather() -> Weather {
        // let weather_host = std::env::var("WEATHER_HOST")
        //     .expect("WEATHER_HOST is not existed in .env");
        let weather_host = "https://wttr.in/";
        let url = format!(
            "{}/{}?format=j1", weather_host, "Hanoi"
        );
        let response = get_api(&url)
            .await
            .expect("cannot get weather data");

        let weather_data: WeatherResponse = serde_json::from_str(&response).expect("cannot parse weather data");

        let current_condition = &weather_data.current_condition[0];
        let temp: f32 = current_condition.temp_C.parse().expect("cannot parse temperature");
        let weather_desc = &current_condition.weatherDesc[0].value;

        match weather_desc.as_str() {
            "Sunny" => Weather::Sunny(temp),
            "Partly cloudy" => Weather::Cloudy(temp),
            "Cloudy" => Weather::Cloudy(temp),
            "Overcast" => Weather::Cloudy(temp),
            "Mist" => Weather::Cloudy(temp),
            "Patchy rain possible" => Weather::Rainy(temp),
            "Patchy snow possible" => Weather::Rainy(temp),
            "Patchy sleet possible" => Weather::Rainy(temp),
            "Patchy freezing drizzle possible" => Weather::Rainy(temp),
            "Thundery outbreaks possible" => Weather::Stormy(temp),
            "Blowing snow" => Weather::Windy(temp),
            "Blizzard" => Weather::Stormy(temp),
            "Fog" => Weather::Cloudy(temp),
            "Freezing fog" => Weather::Cloudy(temp),
            "Patchy light drizzle" => Weather::Rainy(temp),
            "Light drizzle" => Weather::Rainy(temp),
            "Freezing drizzle" => Weather::Rainy(temp),
            "Heavy freezing drizzle" => Weather::Rainy(temp),
            "Patchy light rain" => Weather::Rainy(temp),
            "Light rain" => Weather::Rainy(temp),
            "Moderate rain at times" => Weather::Rainy(temp),
            "Moderate rain" => Weather::Rainy(temp),
            "Heavy rain at times" => Weather::Rainy(temp),
            "Heavy rain" => Weather::Rainy(temp),
            "Light freezing rain" => Weather::Rainy(temp),
            "Moderate or heavy freezing rain" => Weather::Rainy(temp),
            "Light sleet" => Weather::Rainy(temp),
            "Moderate or heavy sleet" => Weather::Rainy(temp),
            "Patchy light snow" => Weather::Rainy(temp),
            "Light snow" => Weather::Rainy(temp),
            "Patchy moderate snow" => Weather::Rainy(temp),
            "Moderate snow" => Weather::Rainy(temp),
            "Patchy heavy snow" => Weather::Rainy(temp),
            "Heavy snow" => Weather::Rainy(temp),
            "Ice pellets" => Weather::Rainy(temp),
            "Light rain shower" => Weather::Rainy(temp),
            "Moderate or heavy rain shower" => Weather::Rainy(temp),
            "Torrential rain shower" => Weather::Rainy(temp),
            "Light sleet showers" => Weather::Rainy(temp),
            "Moderate or heavy sleet showers" => Weather::Rainy(temp),
            "Light snow showers" => Weather::Rainy(temp),
            "Moderate or heavy snow showers" => Weather::Rainy(temp),
            "Light showers of ice pellets" => Weather::Rainy(temp),
            "Moderate or heavy showers of ice pellets" => Weather::Rainy(temp),
            "Patchy light rain with thunder" => Weather::Stormy(temp),
            "Moderate or heavy rain with thunder" => Weather::Stormy(temp),
            "Patchy light snow with thunder" => Weather::Stormy(temp),
            "Moderate or heavy snow with thunder" => Weather::Stormy(temp),
            "Patchy rain nearby" => Weather::Rainy(temp),
            "Light rain, thunderstorm in vicinity, rain with thunderstorm" => Weather::Rainy(temp),
            _ => Weather::Default(temp),
        }
    }

}

pub async fn get_api(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    Ok(response)
}
