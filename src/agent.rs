use std::rc::Rc;
use std::cell::RefCell;
use weather::Weather;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;

/// The agent in the model
#[derive(PartialEq, Serialize, Deserialize)]
pub struct Agent {
    /// The neighbourhood the agent lives in
    #[serde(skip)]
    pub neighbourhood: Rc<Neighbourhood>,

    /// The ID of the neighbourhood
    pub neighbourhood_id: String,

    /// The distance of the agent's commute (categorical).  
    /// This may become deprecated, once commute_length_continuous
    /// has a corresponding cost function
    pub commute_length: JourneyType,

    /// How sensitive the agent is to the weather
    pub weather_sensitivity: f32,

    /// How connected the agent is to its social network
    pub social_connectivity: f32,

    /// How connected the agent is to its neighbourhood
    pub neighbourhood_connectivity: f32,

    /// How the agent is currently travelling to work
    pub current_mode: TransportMode,

    /// How the agent travelled to work on the previous day
    pub last_mode: TransportMode,

    /// Whether the agent owns a bike
    pub owns_bike: bool,

    /// Whether the agent owns a car
    pub owns_car: bool,

    /// The friends of the agent
    #[serde(skip)]
    pub social_network: Vec<Rc<RefCell<Agent>>>,

    /// Neighbours of the agent
    #[serde(skip)]
    pub neighbours: Vec<Rc<RefCell<Agent>>>
}

impl Agent {
    /// Choose a mode of travel
    /// * weather: The current weather
    /// * change_in_weather: true if there has been a change in the weather, false otherwise
    pub fn choose(&mut self, weather: &Weather, change_in_weather: bool) {
        self.current_mode = TransportMode::Car;
    }
}
