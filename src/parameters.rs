use std::fs::File;
use std::io::prelude::*;

/// This stores the parameters of the model
#[derive(Serialize, Deserialize)]
pub struct Parameters {
    /// Total number of years the simulation runs for
    pub total_years: u32,
    /// The number of people in the simulation
    pub number_of_people: u32,
    /// The number of simulations that should take place
    pub number_of_simulations: u32,
    /// How connected an agent is to their social network
    pub social_connectivity: f32,
    /// How connected an agent is to their subculture
    pub subculture_connectivity: f32,
    /// How connected an agent is to their neighbourhood
    pub neighbourhood_connectivity: f32,
    /// The minimum number of links in their social network, and agent should have.
    /// This is the mean number of social network links / 2
    pub number_of_social_network_links: u32,
    /// The minimum number of links in the neighbourhood-wide social network, an agent should have
    /// This is the mean number of links / 2
    pub number_of_neighbour_links: u32,
    /// This is used as a weighting for the habit average, the most recent n days, account
    /// for approximately 86% of the average
    pub days_in_habit_average: u32,

    /// A vec of tuples (mean, sd, weight)
    /// Used for commute length
    pub distributions: Vec<(f64, f64, f64)>
}

impl Parameters {
    /// Loads Parameters from a file
    /// * file: The YAML file storing the serialized parameters
    /// * Returns; The created parameters
    pub fn from_file(mut file: File) -> Self {
        info!("Loading parameters from file");
        let mut file_contents = String::new();

        file.read_to_string(&mut file_contents)
            .expect("There was an error reading the file");

        serde_yaml::from_slice(file_contents.as_bytes())
            .expect("There was an error parsing the file")
    }
}