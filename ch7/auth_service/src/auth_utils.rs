pub fn login(cred: models::Credentials) {
    //authenticate..
    crate::database::get_user();
}

fn logout() {
    // log user out
}

pub mod models;
