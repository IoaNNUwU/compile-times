use std::{collections::HashMap, io::Read};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData {
    name: Option<String>,
    id: i32,
    data: Option<HashMap<String, String>>,
}

impl UserData {
    pub fn new(name: String, id: i32, data: Option<HashMap<String, String>>) -> UserData {
        Self {
            name: Some(name),
            id,
            data,
        }
    }
}

pub fn print_user_data(u: &UserData) {
    let u_json = serde_json::to_string(u).unwrap();
    print!("{u_json}");
}

pub fn read_user_data() -> Option<UserData> {
    let mut buf: [u8; 1024] = std::array::from_fn(|_| 0);
    let n_read = std::io::stdin().read(&mut buf).unwrap();
    let buf = &mut buf[0..n_read];

    serde_json::from_slice(&buf).ok()
}
