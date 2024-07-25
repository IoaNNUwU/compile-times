#[cfg(feature = "mod")]
mod app;

#[cfg(feature = "crate")]
use comp as app;

use app::UserData;

fn main() {

    let data = UserData::new("User1".to_string(), 10, None);
    
    app::print_user_data(&data);

    let data = app::read_user_data();
    std::hint::black_box(data);
}