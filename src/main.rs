use vibing::data_collector::WeatherData;



#[tokio::main]
async fn main() {
    let weather = WeatherData::get_weather().await;

    println!("{:?}", weather);
}