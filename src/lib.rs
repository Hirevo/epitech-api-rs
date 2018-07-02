#![allow(dead_code)]

extern crate chrono;
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod constants;
mod response;

use chrono::prelude::*;
use reqwest::header;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::default::Default;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct EpitechClientBuilder {
    autologin: String,
}

#[derive(Debug, Clone)]
pub struct EpitechClient {
    autologin: String,
    client: reqwest::Client,
    login: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Location {
    Bordeaux,
    LaReunion,
    Lille,
    Lyon,
    Marseille,
    Montpellier,
    Nancy,
    Nantes,
    Nice,
    Paris,
    Rennes,
    Strasbourg,
    Toulouse,
    Berlin,
    Barcelone,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Promo {
    Tek1,
    Tek2,
    Tek3,
}

#[derive(Debug, Clone)]
pub struct StudentListFetchBuilder {
    client: EpitechClient,
    location: Option<Location>,
    promo: Option<Promo>,
    year: u32,
    course: String,
    active: bool,
    offset: u32,
}

#[derive(Debug, Clone)]
pub struct StudentDataFetchBuilder {
    client: EpitechClient,
    login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserEntries {
    pub total: usize,
    pub items: Vec<response::UserEntry>,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Error {
    InvalidStatusCode(u16),
    CookieNotFound,
    UnreachableRemote,
    InternalError,
}

impl EpitechClientBuilder {
    pub fn new() -> EpitechClientBuilder {
        EpitechClientBuilder {
            autologin: String::default(),
        }
    }

    #[inline]
    pub fn autologin<'a, T: Into<String>>(mut self, autologin: T) -> EpitechClientBuilder {
        self.autologin = autologin.into();
        self
    }

    pub fn authenticate(self) -> Result<EpitechClient, Error> {
        let client = match reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
            .build()
        {
            Ok(x) => x,
            Err(_) => return Err(Error::InternalError),
        };
        match client.get(&self.autologin).send() {
            Ok(resp) => match resp.headers().get::<header::SetCookie>() {
                Some(cookie) => {
                    let mut headers = header::Headers::new();
                    let mut new_cookie = header::Cookie::new();
                    for it in cookie.iter() {
                        if it.starts_with("user=") {
                            let whole = it.split(";").nth(0).unwrap();
                            let name = String::from(&whole[0..4]);
                            let value = String::from(&whole[5..]);
                            new_cookie.append(name, value);
                        }
                    }
                    headers.set(new_cookie);
                    let mut client = EpitechClient {
                        autologin: self.autologin.clone(),
                        client: match reqwest::Client::builder().default_headers(headers).build() {
                            Ok(x) => x,
                            Err(_) => return Err(Error::InternalError),
                        },
                        login: String::default(),
                    };
                    match client.fetch_student_data().send() {
                        Some(data) => {
                            client.login = data.login.clone();
                            Ok(client)
                        }
                        None => Err(Error::InternalError),
                    }
                }
                None => Err(Error::CookieNotFound),
            },
            Err(err) => {
                let status = err.status();
                match status {
                    Some(status) => Err(Error::InvalidStatusCode(status.as_u16())),
                    None => Err(Error::UnreachableRemote),
                }
            }
        }
    }
}

impl EpitechClient {
    #[inline]
    pub fn builder() -> EpitechClientBuilder {
        EpitechClientBuilder::new()
    }

    pub fn make_request<T: ToString>(&self, url: T) -> Option<String> {
        let mut string = url.to_string();
        if !string.contains("&format=json") && !string.contains("?format=json") {
            let b = string.contains("?");
            string.push(if b { '&' } else { '?' });
            string.push_str("format=json");
        }
        if !string.starts_with(constants::ENDPOINT) {
            string.insert_str(0, constants::ENDPOINT);
        }
        self.client
            .get(&string)
            .send()
            .and_then(|mut val| val.text())
            .ok()
    }

    pub fn fetch_student_list(&self) -> StudentListFetchBuilder {
        StudentListFetchBuilder::new().client(self.clone())
    }

    pub fn fetch_student_data(&self) -> StudentDataFetchBuilder {
        StudentDataFetchBuilder::new().client(self.clone())
    }

    pub fn fetch_student_netsoul<'a>(
        &self,
        login: &'a str,
    ) -> Option<Vec<response::UserNetsoulEntry>> {
        let url = format!("/user/{}/netsoul", login);
        self.make_request(url)
            .and_then(|text| serde_json::from_str(&text).ok())
    }

    pub fn fetch_own_student_netsoul(&self) -> Option<Vec<response::UserNetsoulEntry>> {
        self.fetch_student_netsoul(self.login.as_ref())
    }

    pub fn fetch_student_notes<'a>(&self, login: &'a str) -> Option<response::UserNotes> {
        let url = format!("/user/{}/notes", login);
        self.make_request(url)
            .and_then(|text| serde_json::from_str(&text).ok())
    }

    pub fn fetch_own_student_notes(&self) -> Option<response::UserNotes> {
        self.fetch_student_notes(self.login.as_ref())
    }

    pub fn fetch_student_binomes<'a>(&self, login: &'a str) -> Option<response::UserBinome> {
        let url = format!("/user/{}/binome", login);
        self.make_request(url)
            .and_then(|text| serde_json::from_str(&text).ok())
    }

    pub fn fetch_own_student_binomes(&self) -> Option<response::UserBinome> {
        self.fetch_student_binomes(self.login.as_ref())
    }
}

impl Default for EpitechClient {
    #[inline]
    fn default() -> EpitechClient {
        EpitechClient {
            autologin: String::default(),
            client: reqwest::Client::new(),
            login: String::default(),
        }
    }
}

impl StudentListFetchBuilder {
    #[inline]
    pub fn new() -> StudentListFetchBuilder {
        StudentListFetchBuilder {
            client: EpitechClient::default(),
            location: None,
            promo: None,
            active: true,
            offset: 0,
            year: Local::now().date().year() as u32,
            course: String::from("bachelor/classic"),
        }
    }

    pub fn send(self) -> Option<Vec<response::UserEntry>> {
        let mut url = String::from(format!("/user/filter/user?offset={}", self.offset));
        match self.location {
            Some(ref location) => url.push_str(format!("&location={}", location).as_ref()),
            None => {}
        };
        match self.promo {
            Some(ref promo) => url.push_str(format!("&promo={}", promo).as_ref()),
            None => {}
        };
        url.push_str(format!("&year={}", self.year).as_ref());
        url.push_str(format!("&course={}", self.course).as_ref());
        url.push_str(format!("&active={}", self.active).as_ref());
        self.client
            .make_request(&url)
            .and_then(|text| serde_json::from_str::<UserEntries>(&text).ok())
            .and_then(|mut v| {
                let state: usize = (self.offset as usize) + v.items.len();
                if state == v.total {
                    Some(v.items)
                } else if state >= v.total {
                    None
                } else {
                    self.offset(state as u32).send().map(|mut x| {
                        v.items.append(&mut x);
                        v.items
                    })
                }
            })
    }

    #[inline]
    pub fn client(mut self, client: EpitechClient) -> StudentListFetchBuilder {
        self.client = client;
        self
    }

    #[inline]
    pub fn location(mut self, location: Location) -> StudentListFetchBuilder {
        self.location = Some(location);
        self
    }

    #[inline]
    pub fn active(mut self, active: bool) -> StudentListFetchBuilder {
        self.active = active;
        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> StudentListFetchBuilder {
        self.offset = offset;
        self
    }

    #[inline]
    pub fn year(mut self, year: u32) -> StudentListFetchBuilder {
        self.year = year;
        self
    }

    #[inline]
    pub fn promo(mut self, promo: Promo) -> StudentListFetchBuilder {
        self.promo = Some(promo);
        self
    }

    #[inline]
    pub fn course<T: Into<String>>(mut self, course: T) -> StudentListFetchBuilder {
        self.course = course.into();
        self
    }
}

impl StudentDataFetchBuilder {
    #[inline]
    pub fn new() -> StudentDataFetchBuilder {
        StudentDataFetchBuilder {
            client: EpitechClient::default(),
            login: None,
        }
    }

    pub fn send(self) -> Option<response::UserData> {
        let url = self.login
            .as_ref()
            .map(|login| format!("/user/{}", login))
            .unwrap_or_else(|| String::from("/user"));
        self.client
            .make_request(url)
            .and_then(|text| serde_json::from_str(&text).ok())
    }

    #[inline]
    pub fn client<'a>(mut self, client: EpitechClient) -> StudentDataFetchBuilder {
        self.client = client;
        self
    }

    #[inline]
    pub fn login<T: Into<String>>(mut self, login: T) -> StudentDataFetchBuilder {
        self.login = Some(login.into());
        self
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Location, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'a> Visitor<'a> for StringVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a string formatted like '<Country>/<City>' (eg. 'FR/STG')"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.to_owned())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }

        deserializer
            .deserialize_string(StringVisitor)
            .and_then(|val| {
                val.parse()
                    .map_err(|_| serde::de::Error::custom("Error deserializing Location."))
            })
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl FromStr for Location {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        for it in constants::LOCATION_TABLE.iter() {
            if string == *it.1 {
                return Ok(it.0.clone());
            }
        }
        Err(())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ret = constants::LOCATION_TABLE
            .get(self)
            .map(|val| *val)
            .unwrap_or("Unknown");
        write!(f, "{}", ret)
    }
}

impl FromStr for Promo {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        for it in constants::PROMO_TABLE.iter() {
            if string == *it.1 {
                return Ok(it.0.clone());
            }
        }
        Err(())
    }
}

impl Display for Promo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ret = constants::PROMO_TABLE
            .get(self)
            .map(|val| *val)
            .unwrap_or("Unknown");
        write!(f, "{}", ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn get_client() -> Result<EpitechClient, Error> {
        let mut contents = String::default();
        std::fs::File::open("test-config.json")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        let val = String::from(
            serde_json::from_str::<serde_json::Value>(&contents)
                .unwrap()
                .as_object()
                .unwrap()
                .get("autologin")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        EpitechClient::builder().autologin(val).authenticate()
    }

    #[test]
    fn auth_unreachable_remote() {
        let ret = EpitechClient::builder().autologin("toto").authenticate();
        assert!(ret.is_err());
        let api = ret.unwrap_err();
        assert!(api == Error::UnreachableRemote);
    }

    #[test]
    fn auth_working_link() {
        let api = get_client();
        assert!(api.is_ok());
    }

    #[test]
    fn fetch_student_list() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_list()
            .location(Location::Strasbourg)
            .promo(Promo::Tek2)
            .year(2017)
            .send();
        assert!(list.is_some());
    }

    #[test]
    fn fetch_all_students_list() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let mut list = Vec::default();
        for promo in constants::PROMO_TABLE.iter() {
            for location in constants::LOCATION_TABLE.iter() {
                println!("{} {}", promo.0, location.0);
                let ret = api.fetch_student_list()
                    .location(location.0.clone())
                    .promo(promo.0.clone())
                    .year(2017)
                    .send();
                list.append(&mut ret.unwrap_or(Vec::default()));
            }
        }
        assert!(list.len() != 0);
    }

    #[test]
    fn fetch_own_student_data() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_data().send();
        assert!(list.is_some());
    }

    #[test]
    fn fetch_other_student_data() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_data()
            .login("nicolas.polomack@epitech.eu")
            .send();
        assert!(list.is_some());
    }

    // #[test]
    // fn fetch_own_student_netsoul() {
    //     let ret = get_client();
    //     assert!(ret.is_ok());
    //     let api = ret.unwrap();
    //     let list = api.fetch_own_student_netsoul();
    //     assert!(list.is_some());
    // }

    #[test]
    fn fetch_other_student_netsoul() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_netsoul("nicolas.polomack@epitech.eu");
        assert!(list.is_some());
    }

    #[test]
    fn fetch_other_student_notes() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_notes("nicolas.polomack@epitech.eu");
        assert!(list.is_some());
    }

    #[test]
    fn fetch_other_student_binomes() {
        let ret = get_client();
        assert!(ret.is_ok());
        let api = ret.unwrap();
        let list = api.fetch_student_binomes("nicolas.polomack@epitech.eu");
        assert!(list.is_some());
    }
}
