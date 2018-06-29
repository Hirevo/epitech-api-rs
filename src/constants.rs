use {Location, Promo};

pub static LOCATION_TABLE: [(Location, &'static str); 15] = [
    (Location::Barcelone, "ES/BAR"),
    (Location::Berlin, "DE/BER"),
    (Location::Bordeaux, "FR/BDX"),
    (Location::LaReunion, "FR/RUN"),
    (Location::Lille, "FR/LIL"),
    (Location::Lyon, "FR/LYN"),
    (Location::Marseille, "FR/MAR"),
    (Location::Montpellier, "FR/MPL"),
    (Location::Nancy, "FR/NCY"),
    (Location::Nantes, "FR/NAN"),
    (Location::Nice, "FR/NCE"),
    (Location::Paris, "FR/PAR"),
    (Location::Rennes, "FR/REN"),
    (Location::Strasbourg, "FR/STG"),
    (Location::Toulouse, "FR/TLS"),
];

pub static PROMO_TABLE: [(Promo, &'static str); 3] = [
    (Promo::Tek1, "tek1"),
    (Promo::Tek2, "tek2"),
    (Promo::Tek3, "tek3"),
];

pub static ENDPOINT: &'static str = "https://intra.epitech.eu";
