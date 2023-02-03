use google_maps::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HotelsAndPOIS {
  hotels: Vec<Waypoint>,
  pois: Vec<Waypoint>,
}

pub struct DistanceTime {
  pub from: String,
  pub to: String,
  pub distance: DirectionsDistance,
  pub duration: DirectionsDuration,
}

impl HotelsAndPOIS {
  pub async fn get_distance_and_time(
    self,
    client: GoogleMapsClient,
  ) -> Result<Vec<DistanceTime>, google_maps::distance_matrix::error::Error> {
    let distance_matrix = client
      .distance_matrix(self.hotels, self.pois)
      .execute()
      .await?;
    let mut distime = vec![];

    for (i, row) in distance_matrix.rows.iter().enumerate() {
      for (j, element) in row.elements.iter().enumerate() {
        distime.push(DistanceTime {
          from: distance_matrix.origin_addresses[i].clone(),
          to: distance_matrix.destination_addresses[j].clone(),
          distance: element.distance.clone().unwrap(),
          duration: element.duration.clone().unwrap(),
        });
      }
    }

    Ok(distime)
  }
}
