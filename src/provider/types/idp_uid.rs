use sonic_rs::Deserialize;

#[derive(Deserialize)]
pub struct GithubUid {
    pub id: i64,
}
