use crate::*;

fn setup_client() -> Result<EpitechClient, EpitechClientError> {
    EpitechClient::builder()
        .autologin(env!("EPITECH_AUTOLOGIN"))
        .authenticate()
}

#[test]
fn auth_unreachable_remote() {
    let ret = EpitechClient::builder().autologin("toto").authenticate();
    assert!(ret.is_err());
    let api = ret.unwrap_err();
    assert!(api == EpitechClientError::UnreachableRemote);
}

#[test]
fn auth_working_link() {
    let api = setup_client();
    assert!(api.is_ok());
}

#[test]
fn fetch_student_list() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api
        .fetch_student_list()
        .location(Location::Strasbourg)
        .promo(Promo::Tek2)
        .year(2018)
        .send();
    assert!(list.is_ok());
}

#[test]
fn fetch_city_list() {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Region {
        code: Location,
        title: String,
        students: String,
    }
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.make_request("/user/filter/location?active=true");
    assert!(list.is_ok());
    let list = list.unwrap();
    let data = json::from_str::<Vec<Region>>(list.as_str());
    assert!(data.is_ok());
}

#[test]
fn fetch_wac_student_list() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api
        .fetch_student_list()
        .location(Location::Strasbourg)
        .promo(Promo::Wac1)
        .year(2018)
        .send();
    assert!(list.is_ok());
}

#[test]
fn fetch_all_students_list() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let mut list = Vec::default();
    for promo in Promo::into_enum_iter() {
        for location in Location::into_enum_iter() {
            println!("{} {}", promo, location);
            let students = api
                .fetch_student_list()
                .location(location)
                .promo(promo)
                .year(2019)
                .send();
            if let Ok(mut students) = students {
                list.append(&mut students);
            }
        }
    }
    assert!(!list.is_empty());
}

#[test]
fn fetch_own_student_data() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.fetch_student_data().send();
    assert!(list.is_ok());
}

#[test]
fn fetch_other_student_data() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api
        .fetch_student_data()
        .login("nicolas.polomack@epitech.eu")
        .send();
    assert!(list.is_ok());
}

// #[test]
// fn fetch_own_student_netsoul() {
//     let ret = setup_client();
//     assert!(ret.is_ok());
//     let api = ret.unwrap();
//     let list = api.fetch_own_student_netsoul();
//     assert!(list.is_some());
// }

#[test]
fn fetch_other_student_netsoul() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.fetch_student_netsoul("nicolas.polomack@epitech.eu");
    assert!(list.is_ok());
}

#[test]
fn fetch_other_student_notes() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.fetch_student_notes("nicolas.polomack@epitech.eu");
    assert!(list.is_ok());
}

#[test]
fn fetch_other_student_binomes() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.fetch_student_binomes("nicolas.polomack@epitech.eu");
    assert!(list.is_ok());
}

// #[test]
#[allow(unused)]
fn fetch_all_gpas() {
    let client = setup_client().unwrap();
    let list = client
        .fetch_student_list()
        .promo(Promo::Tek2)
        .location(Location::Strasbourg)
        .year(2017)
        .send()
        .unwrap();
    let data: Vec<(String, String, String, f32)> = list
        .iter()
        .map(|elem| {
            let ret = client
                .fetch_student_data()
                .login(elem.login.as_str())
                .send();
            if let Err(err) = &ret {
                println!("GPA Fetch: {} [{}]", elem.login, err);
            }
            ret
        })
        .filter(|ret| ret.is_ok())
        .map(|ret| ret.unwrap())
        .map(|data| {
            (
                data.firstname,
                data.lastname,
                data.login,
                data.gpa
                    .expect("No GPA field.")
                    .get(0)
                    .expect("No GPA elements.")
                    .gpa
                    .parse()
                    .expect("Can't map GPA to a float."),
            )
        })
        .collect();
    for (firstname, lastname, login, gpa) in data {
        println!("{} {} [{}]: {}", firstname, lastname, login, gpa);
    }
}

#[test]
fn search_student() {
    let client = setup_client().unwrap();
    let ret = client.search_student("nicolas.poloma");
    assert!(ret.is_ok());
    println!("{:?}", ret.unwrap());
}

#[test]
fn fetch_available_courses() {
    let ret = setup_client();
    assert!(ret.is_ok());
    let api = ret.unwrap();
    let list = api.fetch_available_courses(Location::Strasbourg, 2018, true);
    assert!(list.is_ok());
}
