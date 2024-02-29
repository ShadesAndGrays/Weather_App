## Weather App

This is a small cli weather app built with rust.  
I followed a linkedin [article][tutorial] I stumbled across.  

[tutorial]: https://www.linkedin.com/pulse/building-simple-weather-app-rust-beginners-guide-jenifer-rajendren-nsgpc/?trk=article-ssr-frontend-pulse_more-articles_related-content-card

### Dependencies  
dotenv 0.15.0  
reqwest 0.11  
serde 1.0  
tokio 1.0  

### Usage  

You'll need an API key from [Open weather][ow]  

[ow]: https://openweathermap.org/

Simply sign in and confirm registration  

---
Create a .env file in the project source directory for the API key  

.env
```
API_KEY=YOUR_API_KEY  
```

---

Finally, run the program with `cargo run` in the terminal  
Enter the name of the location  
And done  

---

```
$ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s  
     Running `target/debug/weather_app`  
Enter Location:  
Abuja, NG  
```

```
Weather in Abuja, NG: 30.08°C, overcast clouds  
```
