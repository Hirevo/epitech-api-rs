use serde_json;

use Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub title: String,
    pub login: String,
    pub nom: String,
    pub prenom: String,
    pub picture: Option<String>,
    pub location: Location,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataInfo {
    pub city: Option<UserDataInfoFields>,
    pub telephone: Option<UserDataInfoFields>,
    pub country: Option<UserDataInfoFields>,
    pub birthplace: Option<UserDataInfoFields>,
    pub birthday: Option<UserDataInfoFields>,
    pub facebook: Option<UserDataInfoFields>,
    pub email: Option<UserDataInfoFields>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataInfoFields {
    pub value: String,
    #[serde(default)]
    pub adm: bool,
    #[serde(default)]
    pub public: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataGroup {
    pub title: String,
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataGPA {
    pub gpa: String,
    pub cycle: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataSpice {
    #[serde(default)]
    pub available_spice: Option<String>,
    #[serde(default)]
    pub consumed_spice: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDataNsStat {
    pub active: u32,
    pub idle: u32,
    pub out_active: u32,
    pub out_idle: u32,
    pub nslog_norm: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserData {
    pub login: String,
    pub title: String,
    pub internal_email: String,
    pub lastname: String,
    pub firstname: String,
    pub userinfo: UserDataInfo,
    pub referent_used: bool,
    pub picture: String,
    pub picture_fun: Option<String>,
    pub scolaryear: Option<String>,
    #[serde(default)]
    pub promo: Option<u32>,
    #[serde(default)]
    pub semester: Option<u32>,
    pub location: String,
    pub documents: Option<String>,
    pub userdocs: Option<String>,
    pub shell: Option<String>,
    pub close: bool,
    pub ctime: String,
    pub mtime: String,
    #[serde(default)]
    pub id_promo: Option<String>,
    pub id_history: Option<String>,
    pub course_code: Option<String>,
    pub semester_code: Option<String>,
    pub school_id: Option<String>,
    pub school_code: Option<String>,
    pub school_title: Option<String>,
    pub old_id_promo: Option<String>,
    pub old_id_location: Option<String>,
    pub rights: serde_json::Value,
    pub invited: bool,
    #[serde(default)]
    pub studentyear: Option<u32>,
    pub admin: bool,
    pub editable: bool,
    pub groups: Vec<UserDataGroup>,
    #[serde(default)]
    pub events: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub credits: Option<u32>,
    #[serde(default)]
    pub gpa: Option<Vec<UserDataGPA>>,
    #[serde(default)]
    pub spice: Option<UserDataSpice>,
    #[serde(default)]
    pub nsstat: Option<UserDataNsStat>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNetsoulEntry(u64, f64, f64, f64, f64, f64);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNotes {
    pub modules: Vec<UserNotesModule>,
    pub notes: Vec<UserNotesMark>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNotesModule {
    pub scolaryear: u32,
    pub id_user_history: String,
    pub codemodule: String,
    pub codeinstance: String,
    pub title: String,
    pub date_ins: String,
    pub cycle: String,
    pub grade: String,
    pub credits: f32,
    pub barrage: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserNotesMark {
    pub scolaryear: u32,
    pub codemodule: String,
    pub titlemodule: String,
    pub codeinstance: String,
    pub codeacti: String,
    pub title: String,
    pub date: String,
    pub correcteur: String,
    pub final_note: f32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserBinome {
    pub user: UserBinomeUser,
    pub binomes: Vec<UserBinomeEntry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserBinomeUser {
    pub login: String,
    pub picture: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserBinomeEntry {
    pub login: String,
    pub picture: String,
    pub activities: String,
    pub id_activities: String,
    pub nb_activities: String,
    pub weight: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSearchResultEntry {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub user_type: String,
    pub login: String,
    pub picture_fun: Option<String>,
    pub picture: Option<String>,
    pub course_code: Option<String>,
    pub promo: Option<String>,
    pub course: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableCourseEntry {
    pub students: String,
    pub code: String,
    pub shortcode_school: String,
    pub title: String,
    pub old_title: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailablePromoEntry {
    pub students: String,
    pub promo: String,
    pub promo_deprecated: String,
}
