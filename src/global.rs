use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub static USER_TOKEN: OnceCell<Arc<RwLock<Option<Token>>>> = OnceCell::const_new();

pub async fn clear_user_token() -> anyhow::Result<()> {
    match USER_TOKEN.initialized() {
        true => {
            log::info!("clearing user token");
            let current_token = get_user_token().await?;
            let mut current_token = current_token.write().await;
            *current_token = None;
            Ok(())
        }
        false => Ok(()),
    }
}

pub async fn set_user_token(token: Token) -> anyhow::Result<()> {
    match USER_TOKEN.initialized() {
        true => {
            log::info!("setting new user token");
            let current_token = get_user_token().await?;
            let mut current_token = current_token.write().await;
            *current_token = Some(token);
            Ok(())
        }
        false => match USER_TOKEN.set(Arc::new(RwLock::new(Some(token)))) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("set user token failed: {}", e.to_string()),
        },
    }
}

pub async fn get_user_token() -> anyhow::Result<Arc<RwLock<Option<Token>>>> {
    match USER_TOKEN.get() {
        Some(token) => Ok(token.clone()),
        None => {
            let token = Arc::new(RwLock::new(None));
            match USER_TOKEN.set(token.clone()) {
                Ok(_) => Ok(token),
                Err(e) => anyhow::bail!("get user token failed: {}", e.to_string()),
            }
        }
    }
}

pub static SHOPPING_CART: OnceCell<Arc<RwLock<ShoppingCart>>> = OnceCell::const_new();

pub async fn clear_shopping_cart() -> anyhow::Result<()> {
    match SHOPPING_CART.initialized() {
        true => {
            log::info!("clearing shopping cart");
            let current_cart = get_shopping_cart().await?;
            let mut current_cart = current_cart.write().await;
            current_cart.clear();
            Ok(())
        }
        false => Ok(()),
    }
}

pub async fn get_shopping_cart() -> anyhow::Result<Arc<RwLock<ShoppingCart>>> {
    match SHOPPING_CART.get() {
        Some(cart) => Ok(cart.clone()),
        None => {
            let cart = Arc::new(RwLock::new(ShoppingCart::new()));
            match SHOPPING_CART.set(cart.clone()) {
                Ok(_) => Ok(cart),
                Err(e) => anyhow::bail!("get shopping cart failed: {}", e.to_string()),
            }
        }
    }
}

pub async fn add_item_to_cart(book_id: u32, quantity: u32) -> anyhow::Result<()> {
    let cart = get_shopping_cart().await?;
    let mut cart = cart.write().await;
    cart.add_item(book_id, quantity);
    Ok(())
}

pub async fn set_item_to_cart(book_id: u32, quantity: u32) -> anyhow::Result<()> {
    let cart = get_shopping_cart().await?;
    let mut cart = cart.write().await;
    cart.set_item(book_id, quantity);
    Ok(())
}

pub async fn remove_item_from_cart(book_id: u32) -> anyhow::Result<()> {
    let cart = get_shopping_cart().await?;
    let mut cart = cart.write().await;
    cart.remove_item(book_id);
    Ok(())
}

pub async fn get_cart_total() -> anyhow::Result<u32> {
    let cart = get_shopping_cart().await?;
    let cart = cart.read().await;
    Ok(cart.get_total())
}

pub async fn get_cart_items() -> anyhow::Result<Vec<(u32, u32)>> {
    let cart = get_shopping_cart().await?;
    let cart = cart.read().await;
    Ok(cart.get_total_items())
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Default)]
pub struct ShoppingCart {
    pub book_quantity: HashMap<u32, u32>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        Self {
            book_quantity: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, book_id: u32, quantity: u32) {
        let entry = self.book_quantity.entry(book_id).or_insert(0);
        *entry += quantity;
        if *entry == 0 {
            self.book_quantity.remove(&book_id);
        }
    }

    pub fn set_item(&mut self, book_id: u32, quantity: u32) {
        self.book_quantity.insert(book_id, quantity);
    }

    pub fn remove_item(&mut self, book_id: u32) {
        self.book_quantity.remove(&book_id);
    }

    pub fn get_total(&self) -> u32 {
        self.book_quantity
            .iter()
            .map(|(_, quantity)| quantity)
            .sum()
    }

    pub fn get_total_items(&self) -> Vec<(u32, u32)> {
        self.book_quantity
            .iter()
            .map(|(book_id, quantity)| (*book_id, *quantity))
            .collect()
    }

    pub fn clear(&mut self) {
        self.book_quantity.clear();
    }
}
