use crate::models::user_info::UserInfo;

#[derive(Default)]
pub struct Engine {
    cur_user_info: UserInfo,
}

impl Engine {
    pub fn default() -> Engine {
        Engine {
            ..Default::default()
        }
    }
}

/// 基本数据类型
impl Engine {
    pub fn add_int(&self, left: i32, right: i32) -> i32 {
        left + right
    }

    pub fn add_string(&self, left: &str, right: &str) -> String {
        format!("{}{}", left, right)
    }

    pub fn add_double(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    pub fn add_bool(&self, left: bool, right: bool) -> bool {
        left && right
    }
}

/// 复杂数据类型
impl Engine {
    pub fn set_user_info(&mut self, user_info: UserInfo) {
        self.cur_user_info = user_info;
    }

    pub fn get_user_info(&self) -> UserInfo {
        self.cur_user_info.clone()
    }
}

/// 函数调用
impl Engine {}
