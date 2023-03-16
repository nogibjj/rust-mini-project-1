use reqwest::Error;
use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set");
    let city = "New York"; // You can replace this with your desired city

    let weather_data = fetch_weather_data(&api_key, city).await?;
    print_weather_report(&weather_data);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    weather: Vec<Weather>,
    main: Main,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u32,
}

async fn fetch_weather_data(api_key: &str, city: &str) -> Result<WeatherData, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await?;
    let weather_data: WeatherData = response.json().await?;

    Ok(weather_data)
}

fn print_weather_report(weather_data: &WeatherData) {
    let temp = weather_data.main.temp;
    let humidity = weather_data.main.humidity;
    let description = &weather_data.weather[0].description;

    println!("Weather Report for {}: ", weather_data.name);
    println!("Temperature: {:.2}°C", temp);
    println!("Humidity: {}%", humidity);
    println!("Description: {}", description);
}
