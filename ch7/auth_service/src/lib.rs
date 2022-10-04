mod database;
mod auth_utils;
// how to seperate the modules and sub-modules 
// method #1 
// create the same name folder inside src directory
// and then create `mod.rs` file in 'auth_utils folder)
// copy and paste the contents in the module.
// and then create 'sub-modules name.rs'
// copy and paste the contents in the sub-modules.
// and then go to `mod.rs` the sub-modules declare semi-colone
// also go to lib.rs the modules declare semi-colone

// method #2 (recommand, and striaght forward)
// create the same name folder inside src directory
// and then create the same name file on the src directory. same path
// and then create `the sub-modules name.rs` in the 'auth_utils'folder
// file contents exactly same thing 
use auth_utils::models::Credentials;
// exporting otherside use `use` keyword and `pub` keyword
// pub use auth_utils::models::Credentials;

use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
