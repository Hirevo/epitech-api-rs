use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

use chrono::prelude::*;
use enum_iterator::IntoEnumIterator;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod response;

#[cfg(test)]
mod tests;

use crate::error::Error;

pub static ENDPOINT: &str = "https://intra.epitech.eu";

#[derive(Debug, Clone, Default)]
pub struct ClientBuilder {
    autologin: String,
    retry_count: u32,
}

#[derive(Debug, Clone)]
pub struct Client {
    autologin: String,
    retry_count: u32,
    client: reqwest::Client,
    login: String,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    IntoEnumIterator,
)]
pub enum Location {
    #[serde(rename = "ES/BAR")]
    Barcelone,
    #[serde(rename = "DE/BER")]
    Berlin,
    #[serde(rename = "FR/BDX")]
    Bordeaux,
    #[serde(rename = "FR/RUN")]
    LaReunion,
    #[serde(rename = "FR/LIL")]
    Lille,
    #[serde(rename = "FR/LYN")]
    Lyon,
    #[serde(rename = "FR/MAR")]
    Marseille,
    #[serde(rename = "FR/MPL")]
    Montpellier,
    #[serde(rename = "FR/NCY")]
    Nancy,
    #[serde(rename = "FR/NAN")]
    Nantes,
    #[serde(rename = "FR/NCE")]
    Nice,
    #[serde(rename = "FR/PAR")]
    Paris,
    #[serde(rename = "FR/REN")]
    Rennes,
    #[serde(rename = "FR/STG")]
    Strasbourg,
    #[serde(rename = "FR/TLS")]
    Toulouse,
    #[serde(rename = "BJ/COT")]
    Cotonou,
    #[serde(rename = "AL/TIR")]
    Tirana,
    #[serde(rename = "BE/BRU")]
    Bruxelles,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    IntoEnumIterator,
)]
pub enum Promo {
    #[serde(rename = "tek1")]
    Tek1,
    #[serde(rename = "tek2")]
    Tek2,
    #[serde(rename = "tek3")]
    Tek3,
    #[serde(rename = "wac1")]
    Wac1,
    #[serde(rename = "wac2")]
    Wac2,
    #[serde(rename = "msc3")]
    Msc3,
    #[serde(rename = "msc4")]
    Msc4,
}

#[derive(Debug, Clone, Default)]
pub struct StudentListFetchBuilder {
    client: Client,
    location: Option<Location>,
    promo: Option<Promo>,
    year: u32,
    course: Option<String>,
    active: bool,
    offset: u32,
}

#[derive(Debug, Clone, Default)]
pub struct StudentDataFetchBuilder {
    client: Client,
    login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserEntries {
    pub total: usize,
    pub items: Vec<response::UserEntry>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            autologin: String::default(),
            retry_count: 5,
        }
    }

    #[inline]
    pub fn autologin<T: Into<String>>(mut self, autologin: T) -> ClientBuilder {
        self.autologin = autologin.into();
        self
    }

    #[inline]
    pub fn retry_count(mut self, retry_count: u32) -> ClientBuilder {
        self.retry_count = retry_count;
        self
    }

    pub async fn authenticate(self) -> Result<Client, Error> {
        let client = match reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
        {
            Ok(x) => x,
            Err(_) => return Err(Error::InternalError),
        };
        match client.get(&self.autologin).send().await {
            Ok(response) => {
                let headers = response.headers();
                let cookie = headers
                    .get_all(header::SET_COOKIE)
                    .iter()
                    .filter_map(|it| it.to_str().ok())
                    .find(|cookie| cookie.starts_with("user="))
                    .and_then(|cookie| cookie.split(';').nth(0))
                    .and_then(|cookie| header::HeaderValue::from_str(cookie).ok())
                    .ok_or(Error::CookieNotFound)?;

                let mut headers = header::HeaderMap::new();
                headers.insert(header::COOKIE, cookie);
                let autologin = self.autologin;
                let retry_count = self.retry_count;
                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .build()
                    .map_err(|_| Error::InternalError)?;
                let login = String::default();
                let mut client = Client {
                    autologin,
                    retry_count,
                    client,
                    login,
                };
                let data = client.fetch_student_data().send().await?;
                client.login = data.login.clone();
                Ok(client)
            }
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

impl Client {
    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn make_request<T: ToString>(&self, url: T) -> Result<String, Error> {
        let mut string = url.to_string();
        if !string.contains("&format=json") && !string.contains("?format=json") {
            let b = string.contains('?');
            string.push(if b { '&' } else { '?' });
            string.push_str("format=json");
        }
        if !string.starts_with(ENDPOINT) {
            string.insert_str(0, ENDPOINT);
        }
        for _ in 0..self.retry_count {
            let result = self.client.get(&string).send().await;
            let result = match result {
                Ok(val) => val.text().await,
                Err(err) => Err(err),
            };
            if let Ok(body) = result {
                return Ok(body);
            }
        }
        Err(Error::RetryLimit)
    }

    pub fn fetch_student_list(&self) -> StudentListFetchBuilder {
        StudentListFetchBuilder::new().client(self.clone())
    }

    pub fn fetch_student_data(&self) -> StudentDataFetchBuilder {
        StudentDataFetchBuilder::new().client(self.clone())
    }

    pub async fn fetch_student_netsoul<'a>(
        &self,
        login: &'a str,
    ) -> Result<Vec<response::UserNetsoulEntry>, Error> {
        let url = format!("/user/{}/netsoul", login);
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    pub async fn fetch_own_student_netsoul(
        &self,
    ) -> Result<Vec<response::UserNetsoulEntry>, Error> {
        self.fetch_student_netsoul(self.login.as_ref()).await
    }

    pub async fn fetch_student_notes<'a>(
        &self,
        login: &'a str,
    ) -> Result<response::UserNotes, Error> {
        let url = format!("/user/{}/notes", login);
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    pub async fn fetch_own_student_notes(&self) -> Result<response::UserNotes, Error> {
        self.fetch_student_notes(self.login.as_ref()).await
    }

    pub async fn fetch_student_binomes<'a>(
        &self,
        login: &'a str,
    ) -> Result<response::UserBinome, Error> {
        let url = format!("/user/{}/binome", login);
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    pub async fn fetch_own_student_binomes(&self) -> Result<response::UserBinome, Error> {
        self.fetch_student_binomes(self.login.as_ref()).await
    }

    pub async fn search_student(
        &self,
        login: &str,
    ) -> Result<Vec<response::UserSearchResultEntry>, Error> {
        let url = format!("/complete/user?format=json&contains&search={}", login);
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    pub async fn fetch_available_courses(
        &self,
        location: Location,
        year: u32,
        active: bool,
    ) -> Result<Vec<response::AvailableCourseEntry>, Error> {
        let url = format!(
            "/user/filter/course?format=json&location={}&year={}&active={}",
            location, year, active
        );
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    pub async fn fetch_available_promos(
        &self,
        location: Location,
        year: u32,
        course: &str,
        active: bool,
    ) -> Result<Vec<response::AvailablePromoEntry>, Error> {
        let url = format!(
            "/user/filter/promo?format=json&location={}&year={}&course={}&active={}",
            location, year, course, active
        );
        let response = self.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }
}

impl Default for Client {
    #[inline]
    fn default() -> Client {
        Client {
            autologin: String::default(),
            retry_count: 5,
            client: reqwest::Client::new(),
            login: String::default(),
        }
    }
}

impl StudentListFetchBuilder {
    #[inline]
    pub fn new() -> StudentListFetchBuilder {
        StudentListFetchBuilder {
            client: Client::default(),
            location: None,
            promo: None,
            active: true,
            offset: 0,
            year: Local::now().date().year() as u32,
            course: None,
        }
    }

    fn send_impl(self) -> Pin<Box<dyn Future<Output = Result<Vec<response::UserEntry>, Error>>>> {
        Box::pin(async move {
            let mut url = format!(
                "/user/filter/user?offset={}&year={}&active={}",
                self.offset, self.year, self.active,
            );
            if let Some(ref location) = self.location {
                url = format!("{}&location={}", url, location);
            }
            if let Some(ref promo) = self.promo {
                url = format!("{}&promo={}", url, promo);
            }
            if let Some(ref course) = self.course {
                url = format!("{}&course={}", url, course);
            }
            let response = self.client.make_request(url).await?;
            let mut data = json::from_str::<UserEntries>(&response)?;
            let state: usize = (self.offset as usize) + data.items.len();
            if state == data.total {
                Ok(data.items)
            } else if state >= data.total {
                Err(Error::InternalError)
            } else {
                let mut additional = self.offset(state as u32).send_impl().await?;
                data.items.append(&mut additional);
                Ok(data.items)
            }
        })
    }

    pub async fn send(self) -> Result<Vec<response::UserEntry>, Error> {
        self.send_impl().await
    }

    #[inline]
    pub fn client(mut self, client: Client) -> StudentListFetchBuilder {
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
        self.course = Some(course.into());
        self
    }
}

impl StudentDataFetchBuilder {
    #[inline]
    pub fn new() -> StudentDataFetchBuilder {
        StudentDataFetchBuilder {
            client: Client::default(),
            login: None,
        }
    }

    pub async fn send(self) -> Result<response::UserData, Error> {
        let url = self
            .login
            .map(|login| format!("/user/{}", login))
            .unwrap_or_else(|| String::from("/user"));
        let response = self.client.make_request(url).await?;
        let data = json::from_str(&response)?;
        Ok(data)
    }

    #[inline]
    pub fn client(mut self, client: Client) -> StudentDataFetchBuilder {
        self.client = client;
        self
    }

    #[inline]
    pub fn login<T: Into<String>>(mut self, login: T) -> StudentDataFetchBuilder {
        self.login = Some(login.into());
        self
    }
}

impl FromStr for Location {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "ES/BAR" => Ok(Location::Barcelone),
            "DE/BER" => Ok(Location::Berlin),
            "FR/BDX" => Ok(Location::Bordeaux),
            "FR/RUN" => Ok(Location::LaReunion),
            "FR/LIL" => Ok(Location::Lille),
            "FR/LYN" => Ok(Location::Lyon),
            "FR/MAR" => Ok(Location::Marseille),
            "FR/MPL" => Ok(Location::Montpellier),
            "FR/NCY" => Ok(Location::Nancy),
            "FR/NAN" => Ok(Location::Nantes),
            "FR/NCE" => Ok(Location::Nice),
            "FR/PAR" => Ok(Location::Paris),
            "FR/REN" => Ok(Location::Rennes),
            "FR/STG" => Ok(Location::Strasbourg),
            "FR/TLS" => Ok(Location::Toulouse),
            "BJ/COT" => Ok(Location::Cotonou),
            "AL/TIR" => Ok(Location::Tirana),
            "BE/BRU" => Ok(Location::Bruxelles),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Location::Barcelone => "ES/BAR",
            Location::Berlin => "DE/BER",
            Location::Bordeaux => "FR/BDX",
            Location::LaReunion => "FR/RUN",
            Location::Lille => "FR/LIL",
            Location::Lyon => "FR/LYN",
            Location::Marseille => "FR/MAR",
            Location::Montpellier => "FR/MPL",
            Location::Nancy => "FR/NCY",
            Location::Nantes => "FR/NAN",
            Location::Nice => "FR/NCE",
            Location::Paris => "FR/PAR",
            Location::Rennes => "FR/REN",
            Location::Strasbourg => "FR/STG",
            Location::Toulouse => "FR/TLS",
            Location::Bruxelles => "BE/BRU",
            Location::Cotonou => "BJ/COT",
            Location::Tirana => "AL/TIR",
        };
        write!(f, "{}", repr)
    }
}

impl FromStr for Promo {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "tek1" => Ok(Promo::Tek1),
            "tek2" => Ok(Promo::Tek2),
            "tek3" => Ok(Promo::Tek3),
            "wac1" => Ok(Promo::Wac1),
            "wac2" => Ok(Promo::Wac2),
            "msc3" => Ok(Promo::Msc3),
            "msc4" => Ok(Promo::Msc4),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Promo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Promo::Tek1 => "tek1",
            Promo::Tek2 => "tek2",
            Promo::Tek3 => "tek3",
            Promo::Wac1 => "wac1",
            Promo::Wac2 => "wac2",
            Promo::Msc3 => "msc3",
            Promo::Msc4 => "msc4",
        };
        write!(f, "{}", repr)
    }
}
