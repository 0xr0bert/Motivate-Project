use rand;
use std::collections::HashMap;
use pyo3::prelude::*;

/// The weather for a given day
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Weather {
    Good,
    Bad
}

pub fn register_function(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(make_weather_pattern))?;
    Ok(())
}

/// This generates a weather pattern using a Markov Chain, not for python
/// * transition_matrix: This defines if in state x, the chance of moving to state y is z, where
/// x & y are weather conditions that may be equal
/// * chance_of_rain: The chance of rain on day 0
/// * days: The number of days to generate
/// * Returns: A Vec<Weather> where the index is the day
#[pyfunction]
pub fn make_weather_pattern(
    transition_matrix: &PyDict,
    chance_of_rain: f64,
    days: usize) -> PyResult<Vec<Weather>> {
    
    let matrix: HashMap<Weather, HashMap<Weather, f64>> = transition_matrix.iter()
        .map(|(a, b)| (a.extract().unwrap(), {
            let inner_map: &PyDict = b.extract().unwrap();
            inner_map.iter()
                .map(|(c, d)| (c.extract().unwrap(), d.extract().unwrap()))
                .collect::<HashMap<Weather, f64>>()

        }
        ))
        .collect();

    Ok(make_weather_pattern_nonpy(matrix, chance_of_rain, days))
    
}

/// This generates a weather pattern using a Markov Chain, not for python
/// * transition_matrix: This defines if in state x, the chance of moving to state y is z, where
/// x & y are weather conditions that may be equal
/// * chance_of_rain: The chance of rain on day 0
/// * days: The number of days to generate
/// * Returns: A Vec<Weather> where the index is the day
pub fn make_weather_pattern_nonpy(
    transition_matrix: HashMap<Weather, HashMap<Weather, f64>>,
    chance_of_rain: f64,
    days: usize) -> Vec<Weather> 
{
    // Create an empty weather pattern
    let mut pattern = Vec::with_capacity(days);
    // On day 0, calculate if their is rain
    if rand::random::<f64>() > chance_of_rain {
        pattern.push(Weather::Good);
    } else {
        pattern.push(Weather::Bad);
    }

    // For each day
    for i in 1..days {
        // Using the weather from the previous day,
        // get the probability of good weather, 
        // calculate a random float, if this is less
        // than that probability then the weather for day i
        // is good
        if rand::random::<f64>() < 
            *transition_matrix.get(&pattern[i - 1]).unwrap()
            .get(&Weather::Good).unwrap() {
                pattern.push(Weather::Good);
            } else {
                pattern.push(Weather::Bad);
            }
    }
    pattern
}

impl ToPyObject for Weather {
    fn to_object(&self, py: Python) -> PyObject {
        let string_representation: String = match *self {
            Weather::Good => "GoodWeather".to_string(),
            Weather::Bad => "BadWeather".to_string(),
        };
        PyString::new(py, &string_representation).into()
    }
}

impl IntoPyObject for Weather {
    fn into_object(self, py: Python) -> PyObject {
        let string_representation: String = match self {
            Weather::Good => "GoodWeather".to_string(),
            Weather::Bad => "BadWeather".to_string(),
        };
        PyString::new(py, &string_representation).into()
    }
}

impl<'source> FromPyObject<'source> for Weather {
    fn extract(obj: &'source PyObjectRef) -> PyResult<Self> {
        let string_representation: String = obj.extract().unwrap();
        match string_representation.as_ref() {
            "GoodWeather" => Ok(Weather::Good),
            "BadWeather" => Ok(Weather::Bad),
            _ => Err(pyo3::PyErr::new::<pyo3::exc::TypeError, _>("This is not a weather"))
        }
    }
}