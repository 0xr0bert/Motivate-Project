#![feature(specialization)]
extern crate itertools;
#[macro_use] extern crate maplit;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate pyo3;
extern crate simple_logger;
extern crate serde_yaml;
extern crate rand;
extern crate rayon;
extern crate hashmap_union;

mod weather;
mod transport_mode;
mod subculture;
pub mod statistics;
mod social_network;
mod simulation;
mod scenario;
mod neighbourhood;
mod journey_type;
pub mod intervention;
pub mod gaussian;
pub mod debug;
mod agent;
pub mod agent_generation;

pub use journey_type::JourneyType;
pub use agent::Agent;
pub use weather::Weather;
pub use transport_mode::TransportMode;
pub use subculture::Subculture;
pub use social_network::generate_social_network;
pub use simulation::run;
pub use scenario::Scenario;
pub use neighbourhood::Neighbourhood;

use std::fs::File;
use std::collections::HashMap;
use std::time::SystemTime;
use std::env;
use std::io::Write;
use std::io::prelude::*;
use rayon::prelude::*;
use pyo3::prelude::*;

/// This is the entry point for the application
#[pyfunction]
pub fn main(py: Python, generate: bool, parameters: &Parameters) -> PyResult<()> {
    // Create a new logger for system output
    simple_logger::init().unwrap();

    // Used for monitoring running time
    let t0 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Load parameters from file
    //let parameters = load_parameters_from_file("config/parameters.yaml");


    if generate {
        py.allow_threads(|| {
        generate_and_save_networks(
                parameters.number_of_simulations, 
                parameters.number_of_social_network_links, 
                parameters.number_of_people);
            
            // Create a agents directory to store them in
             std::fs::create_dir_all("config/agents")
                .expect("Failed to create config/agents directory");
        });
    }

    let weather_transition_matrix = hashmap! {
        Weather::Good => hashmap! {
            Weather::Good => 0.886,
            Weather::Bad => 0.114
        },
        Weather::Bad => hashmap! {
            Weather::Good => 0.699,
            Weather::Bad => 0.301
        }
    };

    let weather_pattern = Weather::make_pattern(
        weather_transition_matrix, 0.14, (365 * parameters.total_years) as usize);


    // Run in parallel the simulations
    py.allow_threads(move || (1..=parameters.number_of_simulations)
        .collect::<Vec<u32>>()
        .par_iter()
        .for_each(|id| {
            // Get the network number and load the network
            let network_number = id.to_string();
            let network_file = File::open(format!("config/networks/{}.yaml", network_number))
                .expect("File cannot be opened");

            let network = read_network(network_file);

            let agent_file = if generate {
                File::create(format!("config/agents/{}.yaml", network_number)).expect("File cannot be created")
            } else {
                File::open(format!("config/agents/{}.yaml", network_number)).expect("File cannot be opened")
            };

            simulation::run(id.to_string(),
                        generate,
                        agent_file,
                        File::open("config/scenario.yaml").ok().unwrap(),
                        parameters.total_years,
                        parameters.number_of_people,
                        parameters.social_connectivity,
                        parameters.subculture_connectivity,
                        parameters.neighbourhood_connectivity,
                        parameters.number_of_neighbour_links,
                        parameters.days_in_habit_average,
                        parameters.distributions.clone(),
                        &weather_pattern,
                        network)
                        .unwrap();
    }));

    // Output the running time

    let t1 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    info!("TOTAL RUNNING TIME: {}s", t1 - t0);

    Ok(())
}

#[pymodinit]
fn motivate(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(main))?;
    m.add_class::<Parameters>()?;

    Ok(())
}

/// This stores the parameters of the model
#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct Parameters {
    /// Total number of years the simulation runs for
    #[prop(get, set)]
    total_years: u32,
    /// The number of people in the simulation
    #[prop(get, set)]
    number_of_people: u32,
    /// The number of simulations that should take place
    #[prop(get, set)]
    number_of_simulations: u32,
    /// How connected an agent is to their social network
    #[prop(get, set)]
    social_connectivity: f32,
    /// How connected an agent is to their subculture
    #[prop(get, set)]
    subculture_connectivity: f32,
    /// How connected an agent is to their neighbourhood
    #[prop(get, set)]
    neighbourhood_connectivity: f32,
    /// The minimum number of links in their social network, and agent should have.
    /// This is the mean number of social network links / 2
    #[prop(get, set)]
    number_of_social_network_links: u32,
    /// The minimum number of links in the neighbourhood-wide social network, an agent should have
    /// This is the mean number of links / 2
    #[prop(get, set)]
    number_of_neighbour_links: u32,
    /// This is used as a weighting for the habit average, the most recent n days, account
    /// for approximately 86% of the average
    #[prop(get, set)]
    days_in_habit_average: u32,

    /// A vec of tuples (mean, sd, weight)
    /// Used for commute length
    #[prop(get, set)]
    distributions: Vec<(f64, f64, f64)>,

    // This is for reference only
    // #[prop(get)]
    // age: HashMap<u64, u64>,
}

#[pymethods]
impl Parameters {
    #[new]
    pub fn __new__(obj: &PyRawObject, file_path: &str) -> PyResult<()> {
        obj.init(|_| {
            load_parameters_from_file(file_path)
        })
    }

    // This is for reference only
    // #[setter]
    // pub fn set_age(&mut self, x: &PyDict) -> PyResult<()> {
    //     self.age = x.iter()
    //         .map(|(a, b)| (a.extract().unwrap(), b.extract().unwrap()))
    //         .collect();
    //     Ok(())
    // }

    pub fn write_to_file(&self, file_path: &str) -> PyResult<()> {
        let mut file = File::create(file_path)?;
        let yaml = serde_yaml::to_string(self).ok().unwrap();
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}

/// Loads Parameters from a file
/// * file: The YAML file storing the serialized parameters
/// * Returns; The created parameters
pub fn load_parameters_from_file(file_path: &str) -> Parameters {
    info!("Loading parameters from file");
    let mut file = File::open(file_path)
        .expect("Failed to open parameters file");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .expect("There was an error reading the file");

    serde_yaml::from_slice(file_contents.as_bytes())
        .expect("There was an error parsing the file")
}

/// This generates a social network, and saves it them to YAML files in the networks/ subdirectory
/// * number_of_simulations_per_scenario: One network is generated per scenario
/// * number_of_social_network_links: The minimum number of links each person in the social network has
/// * number_of_people: The number of people in the simulation
pub fn generate_and_save_networks(
    number_of_simulations_per_scenario: u32, 
    number_of_social_network_links: u32,
    number_of_people: u32) 
{
    // Generate as many social networks as number of simulations per scenario
    let numbers: Vec<u32> = (0..number_of_simulations_per_scenario).collect();
    // Get the networks stored as a YAML file
    let networks: Vec<String> = numbers
        .par_iter()
        .map(|_| serde_yaml::to_string(&social_network::generate_social_network(
            number_of_social_network_links, number_of_people)).unwrap())
        .collect();

    // Create a networks directory to store them in
    std::fs::create_dir_all("config/networks")
        .expect("Failed to create config/networks directory");

    // For each network, save the network to a file
    networks
        .par_iter()
        .enumerate()
        .for_each(|(i, item)| {
            let mut file = std::fs::File::create(format!("config/networks/{}.yaml", i+1)).ok().unwrap();
            file.write_all(item.as_bytes()).ok();
        });
    
    info!("Generating networks complete")
}

/// Read a social network from a file
/// * file: An input file in YAML mapping ids to a list of ids
/// * Returns: A HashMap mapping ids, to the ids of their friends
pub fn read_network(mut file: File) -> HashMap<u32, Vec<u32>> {
    info!("READING NETWORK");

    // Create a new String (heap allocated) to store the contents of the file
    let mut file_contents = String::new();

    // Read the file into the String
    file.read_to_string(&mut file_contents)
        .expect("There was an error reading the file");

    // Deserialize the network
    serde_yaml::from_slice(file_contents.as_bytes())
        .expect("There was an error parsing the file")
}