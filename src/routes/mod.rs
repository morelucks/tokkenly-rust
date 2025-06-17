pub mod hello_user;
pub use hello_user::*;

pub mod home;
pub use home::*;

pub mod create_user;
pub use create_user::*;
fn logging(path: &str) {
    println!("{}", path);
}
