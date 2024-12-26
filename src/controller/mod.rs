mod admin;
mod book;
mod index;
mod order;
mod purchase_order;
mod shortage;
mod supplier;
mod user;

pub use admin::{
    admin_book_add, admin_book_detail, admin_book_update, admin_customer_balance,
    admin_customer_credit, admin_customer_list, admin_detail, admin_location_list, admin_login,
    admin_order_list, admin_order_ship_auto, admin_register, admin_stock_change,
    admin_stock_transfer,
};
pub use book::{
    author_list, book_authors_search, book_detail, book_keywords_search, book_list,
    book_title_search, keyword_list, publisher_list, series_list,
};
pub use index::homepage;
pub use order::{order_create, order_detail, order_history};
pub use purchase_order::{purchase_order_detail, purchase_order_list};
pub use shortage::shortage_create;
pub use supplier::{supplier_list, supplier_profile};
pub use user::{credit_rule, login, register, user_detail, user_logout, user_profile, user_update};
