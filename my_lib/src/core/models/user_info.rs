#[derive(Clone, Default)]
pub struct UserInfo {
    id: String,
    name: String,
    age: i32,
}

impl UserInfo {
    pub fn new(id: &str, name: &str, age: i32) -> UserInfo {
        UserInfo {
            id: id.to_string(),
            name: name.to_string(),
            age,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.id
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }
}
