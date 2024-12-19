use std::rc::{Rc, Weak};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,
    pub tag: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Default)]
pub struct WeakUserToken {
    inner: Weak<RwLock<Option<Token>>>,
}

impl WeakUserToken {
    pub fn upgrade(&self) -> Option<UserToken> {
        match self.inner.upgrade() {
            Some(inner) => Some(UserToken { inner }),
            None => None,
        }
    }

    pub fn unwrap(&self) -> UserToken {
        self.upgrade().unwrap()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserToken {
    inner: Rc<RwLock<Option<Token>>>,
}

unsafe impl Send for UserToken {}
unsafe impl Sync for UserToken {}

impl UserToken {
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

    pub fn as_weak(&self) -> WeakUserToken {
        WeakUserToken {
            inner: Rc::downgrade(&self.inner),
        }
    }
}
