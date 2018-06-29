extern crate serde;

use Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    title: String,
    login: String,
    nom: String,
    prenom: String,
    picture: Option<String>,
    location: Location
}
