use std::{env, error::Error, fs::File, process};

use dotenv::dotenv;
use gmaps_distance::*;
use google_maps::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let api_key = match env::var("GOOGLE_API_KEY") {
    Ok(key) => key,
    Err(_) => {
      eprintln!("No Google api key provided!");
      process::exit(1);
    }
  };
  let google_maps_client = GoogleMapsClient::new(&api_key);

  let file = File::open("sample.json")?;
  let data: HotelsAndPOIS = serde_json::from_reader(file)?;

  let distimes = data.get_distance_and_time(google_maps_client).await?;

  for distime in distimes {
    println!(
      "From: {}\nTo: {}\nDistance: {}m\nDuration: {}",
      distime.from, distime.to, distime.distance.value, distime.duration.text
    )
  }

  Ok(())
}
