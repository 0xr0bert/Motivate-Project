use pyo3::prelude::*;

/// The Transport Modes that can be taken by agents
#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TransportMode {
    Car,
    PublicTransport,
    Cycle,
    Walk
}

impl ToPyObject for TransportMode {
    fn to_object(&self, py: Python) -> PyObject {
        let string_representation: String = match *self {
            TransportMode::Car => "Car".to_string(),
            TransportMode::PublicTransport => "Public Transport".to_string(),
            TransportMode::Cycle => "Cycle".to_string(),
            TransportMode::Walk => "Walk".to_string()
        };
        PyString::new(py, &string_representation).into()
    }
}

impl<'source> FromPyObject<'source> for TransportMode {
    fn extract(obj: &'source PyObjectRef) -> PyResult<Self> {
        let string_representation: String = obj.extract().unwrap();
        match string_representation.as_ref() {
            "Car" => Ok(TransportMode::Car),
            "Public Tranpsort" => Ok(TransportMode::PublicTransport),
            "Cycle" => Ok(TransportMode::Cycle),
            "Walk" => Ok(TransportMode::Walk),
            _ => Err(pyo3::PyErr::new::<pyo3::exc::TypeError, _>("This is not a transport mode"))
        }
    }
}
