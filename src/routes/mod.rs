pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

pub mod create_user;
pub use create_user::*;

pub mod auth;
pub use auth::*;

fn logging(path: &str) {
    println!("{}", path);
}
