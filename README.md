## Weather App

This is a small cli weather app build with rust.
I followed a linkedin [article][tutorial] I stumbled accross.

[tutorial]: https://www.linkedin.com/pulse/building-simple-weather-app-rust-beginners-guide-jenifer-rajendren-nsgpc/?trk=article-ssr-frontend-pulse_more-articles_related-content-card

### Dependencies

dotenv 0.15.0
reqwest 0.11
serde 1.0
tokio 1.0

### Usage

You'll need an API key from [Open weather][ow]

[ow]: https://openweathermap.org/

simply sign in and confirm registration to recieve API Key

---
Create a .env file in the project source dir for the API key

.env
```
API_KEY=YOUR_API_KEY
```

---

Finally, run the program with `Cargo run` in the terminal
Enter the name of the location
and done

---

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/weather_app`
Enter Location: 
America
```

```
Weather in America: 300.17°C, clear sky
```
