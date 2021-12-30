use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth(pub String);

/**--------------------------------------------
 *               Utils
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetails {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
}

/**--------------------------------------------
 *               Login route
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub auth: Auth,
    pub user_details: UserDetails,
}

/**--------------------------------------------
 *               Create user route
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateResponse {
    pub auth: Auth,
    pub user_details: UserDetails,
}

/**--------------------------------------------
 *               Get user details route
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub user_details: UserDetails,
}

/**--------------------------------------------
 *               Update user info route
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub old_password: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateResponse {
    pub user_details: UserDetails,
}

/**--------------------------------------------
 *               Delete user route
 *---------------------------------------------**/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub user_details: UserDetails,
}
