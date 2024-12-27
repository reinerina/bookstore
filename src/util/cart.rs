use std::collections::HashMap;
use std::rc::{Rc, Weak};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct Cart {
    book_quantity: HashMap<u32, (u32, u32, u32)>,
}

#[derive(Debug, Clone, Default)]
pub struct WeakShoppingCart {
    inner: Weak<RwLock<Cart>>,
}

impl WeakShoppingCart {
    pub fn upgrade(&self) -> Option<ShoppingCart> {
        match self.inner.upgrade() {
            Some(inner) => Some(ShoppingCart { inner }),
            None => None,
        }
    }

    pub fn unwrap(&self) -> ShoppingCart {
        self.upgrade().unwrap()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShoppingCart {
    inner: Rc<RwLock<Cart>>,
}

unsafe impl Send for ShoppingCart {}
unsafe impl Sync for ShoppingCart {}

impl ShoppingCart {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_item(&self, book_id: u32, quantity: u32) {
        let mut inner = self.inner.write().await;
        let cart = &mut inner.book_quantity;
        let cart_quantity = cart.entry(book_id).or_insert((0, 0, 0));
        cart_quantity.0 += quantity;
    }

    pub async fn add_item_default(&self, book_id: u32) {
        let mut inner = self.inner.write().await;
        let cart = &mut inner.book_quantity;
        cart.entry(book_id).or_insert((0, 0, 0));
    }

    pub async fn set_item(&self, book_id: u32, quantity: u32) {
        let mut inner = self.inner.write().await;
        let cart = &mut inner.book_quantity;
        let supplier_id = cart.get(&book_id).unwrap_or(&(0, 0, 0)).1;
        let supplier_index = cart.get(&book_id).unwrap_or(&(0, 0, 0)).2;
        cart.insert(book_id, (quantity, supplier_id, supplier_index));
    }

    pub async fn set_item_supplier(&self, book_id: u32, supplier_id: u32, supplier_index: u32) {
        let mut inner = self.inner.write().await;
        let cart = &mut inner.book_quantity;
        let quantity = cart.get(&book_id).unwrap_or(&(0, 0, 0)).0;
        cart.insert(book_id, (quantity, supplier_id, supplier_index));
    }

    pub async fn remove_item(&self, book_id: u32) {
        let mut inner = self.inner.write().await;
        let cart = &mut inner.book_quantity;
        cart.remove(&book_id);
    }

    pub async fn get_total(&self) -> u32 {
        let inner = self.inner.read().await;
        let cart = &inner.book_quantity;
        cart.iter().map(|(_, v)| v.0).sum()
    }

    pub async fn get_total_items(&self) -> Vec<(u32, (u32, u32, u32))> {
        let inner = self.inner.read().await;
        let cart = &inner.book_quantity;
        cart.iter().map(|(k, v)| (*k, *v)).collect()
    }

    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.book_quantity.clear();
    }

    pub fn as_weak(&self) -> WeakShoppingCart {
        WeakShoppingCart {
            inner: Rc::downgrade(&self.inner),
        }
    }
}
