#![allow(dead_code)]


use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct  TempData{
    temp:f64
}

#[derive(Deserialize)]
struct WeatherData{
    main:TempData,
    weather:Vec<WeatherDescription>
}

#[derive(Deserialize)]
struct WeatherDescription{
    description:String
}

#[tokio::main]
async fn main(){
    dotenv().ok();
    let open_weather_api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    println!("Enter Location: ");
    // User input
    let mut city = String::new();

    std::io::stdin().read_line(&mut city).expect("Failed to read line");

   let _ = tokio::spawn(fetch_weather(open_weather_api_key, city.clone())).await.expect("Could not fetch fetch Weather");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

}

async fn fetch_weather(api_key:String,city:String) -> Result<(),reqwest::Error>{
    // First initialize url with city location and api key
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",city.trim(),api_key);
    // Sendind request to openweather
    let response = reqwest::get(&url).await?;
    if response.status().is_success(){
        let weather_data:WeatherData = response.json().await?;
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("Weather in {}: {:.2}°C, {}", city.trim(), temperature, description);       
    }
    else{
        println!("Error: {}",response.status());
    }
    Ok(())
}
