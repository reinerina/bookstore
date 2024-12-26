use std::rc::{Rc, Weak};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Default)]
pub struct WeakAdminToken {
    inner: Weak<RwLock<Option<Token>>>,
}

unsafe impl Send for WeakAdminToken {}
unsafe impl Sync for WeakAdminToken {}

impl WeakAdminToken {
    pub fn upgrade(&self) -> Option<AdminToken> {
        match self.inner.upgrade() {
            Some(inner) => Some(AdminToken { inner }),
            None => None,
        }
    }

    pub fn unwrap(&self) -> AdminToken {
        self.upgrade().unwrap()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AdminToken {
    inner: Rc<RwLock<Option<Token>>>,
}

unsafe impl Send for AdminToken {}
unsafe impl Sync for AdminToken {}

impl AdminToken {
    pub fn new(token: Token) -> Self {
        Self {
            inner: Rc::new(RwLock::new(Some(token))),
        }
    }

    pub async fn set(&self, token: Token) {
        let mut inner = self.inner.write().await;
        *inner = Some(token);
    }

    pub async fn get(&self) -> Option<Token> {
        let inner = self.inner.read().await;
        inner.clone()
    }

    pub async fn clear(&self) {
        let mut inner = self.inner.write().await;
        *inner = None;
    }

    pub fn as_weak(&self) -> WeakAdminToken {
        WeakAdminToken {
            inner: Rc::downgrade(&self.inner),
        }
    }
}
