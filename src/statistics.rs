use std::collections::HashMap;
use agent::Agent;
use transport_mode::TransportMode;
use journey_type::JourneyType;
use neighbourhood::Neighbourhood;
use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;

/// Counts the number of agents who take an active mode
/// * agents: The agents to count from
/// * Returns: The number of agents who's current_mode is either Walk or Cycle
pub fn count_active_mode(agents: &[Rc<RefCell<Agent>>]) -> usize {
    agents
        .iter()
        .filter(|&a| a.borrow().current_mode == TransportMode::Walk
            || a.borrow().current_mode == TransportMode::Cycle)
        .count()
}

/// Counts the number of agents who take an active mode grouped by commute length
/// * agents: The agents to count from
/// * Returns: A Map: JourneyType -> The number of agent's who's current mode is Walk or Cycle 
pub fn count_active_mode_by_commute_length(agents: &[Rc<RefCell<Agent>>]) -> HashMap<JourneyType, usize> {
    agents
        .iter()
        .map(|agent| (agent.borrow().commute_length, Rc::clone(agent)))
        .into_group_map()
        .into_iter()
        .map(|(journey_type, grouped_agents)| (journey_type, count_active_mode(&grouped_agents)))
        .collect()
}

/// Counts the number of agents who take an active mode grouped by neighbourhood
/// * neighbourhoods: The neighbourhoods to count from
/// * Returns: A Map: Neighbourhood -> The number of agent's who's current mode is Walk or Cycle 
pub fn count_active_mode_by_neighbourhood (neighbourhoods: &[Rc<Neighbourhood>]) -> HashMap<Rc<Neighbourhood>, usize> {
    neighbourhoods
        .iter()
        .map(|neighbourhood| (Rc::clone(neighbourhood), count_active_mode(&neighbourhood.residents.borrow())))
        .collect()
}
