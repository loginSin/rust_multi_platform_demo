#[derive(Clone, Default)]
pub struct UserInfo {
    id: String,
    name: String,
    age: i32,
}

impl UserInfo {
    pub fn new(id: String, name: String, age: i32) -> UserInfo {
        UserInfo { id, name, age }
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
