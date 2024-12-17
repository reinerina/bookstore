use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Default)]
pub struct LogoutRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct UserDetailRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct BookListRequest;

#[derive(Serialize, Debug, Default)]
pub struct BookDetailRequest;

#[derive(Serialize, Debug, Default)]
pub struct OrderCreateRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub items: Vec<(u32, u32)>,
}

#[derive(Serialize, Debug, Default)]
pub struct OrderHistoryRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}
