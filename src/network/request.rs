use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub name: String,
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
pub struct UserUpdateRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub address: String,
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

#[derive(Serialize, Debug, Default)]
pub struct BookTitleSearchRequest {
    pub title: String,
}

#[derive(Serialize, Debug, Default)]
pub struct BookKeywordsSearchRequest {
    pub keywords: String,
}

#[derive(Serialize, Debug, Default)]
pub struct BookAuthorsSearchRequest {
    pub authors: String,
}

#[derive(Serialize, Debug, Default)]
pub struct OrderDetailRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Serialize, Debug, Default)]
pub struct OrderPaymentRequest {
    pub token: String,
    pub tag: String,
    pub nonce: String,
    pub status: String,
}
