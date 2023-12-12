use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AmdProcessorTypeRequest {
    pub id: i64,
    pub architecture: std::string::String,
    pub fabrication_nm: std::string::String,
    pub family: std::string::String,
    pub release_date: String,
    pub codename: std::string::String,
    pub model_group: std::string::String,
    pub cores: std::string::String,
    pub smt: std::string::String,
    pub clock_rate_mhz: std::string::String,
    pub bus_speed_type: std::string::String,
    pub cache_l1: std::string::String,
    pub cache_l2: std::string::String,
    pub cache_l3: std::string::String,
    pub socket: std::string::String,
    pub memory_controller: std::string::String,
    pub simd: std::string::String,
    pub speed_power: std::string::String,
    pub other: std::string::String,
    pub changes: std::string::String,
}
