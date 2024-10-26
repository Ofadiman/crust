use std::sync::Mutex;

use crate::users_domain;

pub struct State {
    pub users: Mutex<Vec<users_domain::User>>,
}
