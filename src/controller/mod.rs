mod admin;
mod book;
mod index;
mod order;
mod purchase_order;
mod supplier;
mod user;

pub use admin::{admin_login, admin_register};
pub use book::{book_detail, book_keywords_search, book_list, book_title_search};
pub use index::homepage;
pub use order::{order_create, order_detail, order_history};
pub use supplier::supplier_profile;
pub use user::{credit_rule, login, register, user_detail, user_logout, user_profile};
