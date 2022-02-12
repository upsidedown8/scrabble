use crate::auth::Auth;
use serde::{Deserialize, Serialize};

//--------------------------------------------
//               Utils
//--------------------------------------------
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDetails {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
}

//--------------------------------------------
//               Login route
//--------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub auth: Auth,
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Create user route
//--------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct SignUp {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignUpResponse {
    pub auth: Auth,
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Get user details route
//--------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub auth: Auth,
    pub user_details: UserDetails,
}

//--------------------------------------------
//               Update user info route
//--------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccount {
    pub old_password: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountResponse {
    pub auth: Auth,
}

//--------------------------------------------
//               Delete user route
//--------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccount {
    pub password: String,
}
