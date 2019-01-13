use std::collections::HashMap;
use itertools::Itertools;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use weather::Weather;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use hashmap_union::{union_of, intersection_of};

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

    /// The distance of the agent's commute.  
    /// This is currently unused, but should correspond to the
    /// categorical commute_length
    pub commute_length_continuous: f64,

    /// How sensitive the agent is to the weather
    pub weather_sensitivity: f32,
    
    /// How consistent the agent is (used as a weighting for habit)
    pub consistency: f32,

    /// How connected the agent is to its social network
    pub social_connectivity: f32,

    /// How connected the agent is to its neighbourhood
    pub neighbourhood_connectivity: f32,

    /// The weight used for the average
    pub average_weight: f32,

    /// The habit of the agent, mapping the TransportMode, to a
    /// recency-weighted average where 1 was used, if the TransportMode
    /// was used on a given day, 0 if it was not.
    pub habit: HashMap<TransportMode, f32>,

    /// How the agent is currently travelling to work
    pub current_mode: TransportMode,

    /// How the agent travelled to work on the previous day
    pub last_mode: TransportMode,

    /// The maximum of the joint effects of social network
    /// and neighbours, appropriately weighted
    pub norm: TransportMode,

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
    /// Updates the habit, should be called at the start of each day,
    /// Also updates last_mode
    pub fn update_habit(&mut self) {
        self.last_mode = self.current_mode;
    }

    /// Choose a mode of travel
    /// * weather: The current weather
    /// * change_in_weather: true if there has been a change in the weather, false otherwise
    pub fn choose(&mut self, weather: &Weather, change_in_weather: bool) {
        self.current_mode = TransportMode::Car;
    }
}

/// Counts in a subgroup of Agents, the percentage of people taking each travel mode
/// * agents: the subgroup of agents
/// * weight: the weight to apply to the percentage
fn count_in_subgroup(agents: &[Rc<RefCell<Agent>>], weight: f32) -> HashMap<TransportMode, f32> {
    // Group them by travel mode, then calculate the percentage (multiplied by the weight)
    let agents_size = agents.len() as f32;
    agents
        .iter()
        .map(|x| (x.borrow().last_mode, 1))
        .into_group_map()
        .iter()
        .map(|(&k, v)| (k, (v.len() as f32) * weight / agents_size))
        .collect()
}
