/// A categorical distance for commute
#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum JourneyType {
    LocalCommute,
    CityCommute,
    DistantCommute
}