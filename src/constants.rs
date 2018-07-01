extern crate lazy_static;

use std::collections::HashMap;
use {Location, Promo};

lazy_static! {
    pub static ref LOCATION_TABLE: HashMap<Location, &'static str> = {
        let mut map = HashMap::with_capacity(15);
        map.insert(Location::Barcelone, "ES/BAR");
        map.insert(Location::Berlin, "DE/BER");
        map.insert(Location::Bordeaux, "FR/BDX");
        map.insert(Location::LaReunion, "FR/RUN");
        map.insert(Location::Lille, "FR/LIL");
        map.insert(Location::Lyon, "FR/LYN");
        map.insert(Location::Marseille, "FR/MAR");
        map.insert(Location::Montpellier, "FR/MPL");
        map.insert(Location::Nancy, "FR/NCY");
        map.insert(Location::Nantes, "FR/NAN");
        map.insert(Location::Nice, "FR/NCE");
        map.insert(Location::Paris, "FR/PAR");
        map.insert(Location::Rennes, "FR/REN");
        map.insert(Location::Strasbourg, "FR/STG");
        map.insert(Location::Toulouse, "FR/TLS");
        map
    };
    pub static ref PROMO_TABLE: HashMap<Promo, &'static str> = {
        let mut map = HashMap::with_capacity(3);
        map.insert(Promo::Tek1, "tek1");
        map.insert(Promo::Tek2, "tek2");
        map.insert(Promo::Tek3, "tek3");
        map
    };
}

pub static ENDPOINT: &'static str = "https://intra.epitech.eu";
