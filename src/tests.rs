use enum_iterator::IntoEnumIterator;
use futures::future;
use futures::future::FutureExt;
use serde::{Deserialize, Serialize};

use crate::{Client, Error, Location, Promo};

async fn setup_client() -> Result<Client, Error> {
    Client::builder()
        .autologin(env!("EPITECH_AUTOLOGIN"))
        .authenticate()
        .await
}

#[tokio::test]
async fn auth_unreachable_remote() {
    let client = Client::builder().autologin("toto").authenticate().await;
    assert!(client.is_err());
    assert_eq!(client.unwrap_err(), Error::UnreachableRemote);
}

#[tokio::test]
async fn auth_working_link() {
    let client = setup_client().await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn fetch_student_list() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_list()
        .location(Location::Strasbourg)
        .promo(Promo::Tek2)
        .year(2020)
        .send()
        .await;
    assert!(list.is_ok());
}

#[tokio::test]
async fn fetch_city_list() {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Region {
        code: Location,
        title: String,
        students: String,
    }
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .make_request("/user/filter/location?active=true")
        .await;
    assert!(list.is_ok());
    let list = list.unwrap();
    let data = json::from_str::<Vec<Region>>(list.as_str());
    assert!(data.is_ok());
}

#[tokio::test]
async fn fetch_wac_student_list() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_list()
        .location(Location::Strasbourg)
        .promo(Promo::Wac1)
        .year(2018)
        .send()
        .await;
    assert!(list.is_ok());
}

#[tokio::test]
async fn fetch_all_students_list() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let mut list = Vec::default();
    for promo in Promo::into_enum_iter() {
        for location in Location::into_enum_iter() {
            println!("{} {}", promo, location);
            let students = client
                .fetch_student_list()
                .location(location)
                .promo(promo)
                .year(2019)
                .send()
                .await;
            if let Ok(mut students) = students {
                list.append(&mut students);
            }
        }
    }
    assert!(!list.is_empty());
}

#[tokio::test]
async fn fetch_own_student_data() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client.fetch_student_data().send().await;
    assert!(list.is_ok());
}

#[tokio::test]
async fn fetch_other_student_data() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_data()
        .login("nicolas.polomack@epitech.eu")
        .send()
        .await;
    assert!(list.is_ok());
}

// #[tokio::test]
// async fn fetch_own_student_netsoul() {
//     let client = setup_client().await;
//     assert!(client.is_ok());
//     let client = client.unwrap();
//     let list = client.fetch_own_student_netsoul().await;
//     assert!(list.is_some());
// }

#[tokio::test]
async fn fetch_other_student_netsoul() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_netsoul("nicolas.polomack@epitech.eu")
        .await;
    assert!(list.is_ok());
}

#[tokio::test]
async fn fetch_other_student_notes() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_notes("nicolas.polomack@epitech.eu")
        .await;
    assert!(list.is_ok());
}

#[tokio::test]
async fn fetch_other_student_binomes() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_binomes("nicolas.polomack@epitech.eu")
        .await;
    assert!(list.is_ok());
}

// #[tokio::test]
#[allow(unused)]
async fn fetch_all_gpas() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_student_list()
        .promo(Promo::Tek2)
        .location(Location::Strasbourg)
        .year(2017)
        .send()
        .await
        .unwrap();
    let futures = list.iter().map(|elem| {
        client
            .fetch_student_data()
            .login(elem.login.as_str())
            .send()
            .inspect(move |result| {
                if let Err(ref err) = result {
                    println!("GPA Fetch: {} [{}]", elem.login, err);
                }
            })
    });
    let results = future::join_all(futures).await;
    let data: Vec<(String, String, String, f32)> = results
        .into_iter()
        .flatten()
        .map(|data| {
            let firstname = data.firstname;
            let lastname = data.lastname;
            let login = data.login;
            let gpa = data
                .gpa
                .expect("No GPA field.")
                .get(0)
                .expect("No GPA elements.")
                .gpa
                .parse()
                .expect("Can't map GPA to a float.");
            (firstname, lastname, login, gpa)
        })
        .collect();
    for (firstname, lastname, login, gpa) in data {
        println!("{} {} [{}]: {}", firstname, lastname, login, gpa);
    }
}

#[tokio::test]
async fn search_student() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let result = client.search_student("nicolas.poloma").await;
    assert!(result.is_ok());
    println!("{:?}", result.unwrap());
}

#[tokio::test]
async fn fetch_available_courses() {
    let client = setup_client().await;
    assert!(client.is_ok());
    let client = client.unwrap();
    let list = client
        .fetch_available_courses(Location::Strasbourg, 2018, true)
        .await;
    assert!(list.is_ok());
}
