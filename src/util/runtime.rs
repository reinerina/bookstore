use std::future::Future;
use std::rc::{Rc, Weak};
use tokio::runtime;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct WeakRuntime {
    inner: Weak<runtime::Runtime>,
}

impl WeakRuntime {
    pub fn upgrade(&self) -> Option<Runtime> {
        match self.inner.upgrade() {
            Some(inner) => Some(Runtime { inner }),
            None => None,
        }
    }

    pub fn unwrap(&self) -> Runtime {
        self.upgrade().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Runtime {
    inner: Rc<runtime::Runtime>,
}

impl Runtime {
    pub fn new() -> anyhow::Result<Self> {
        match runtime::Builder::new_multi_thread().enable_all().build() {
            Ok(runtime) => {
                let runtime = Rc::new(runtime);
                Ok(Self { inner: runtime })
            }
            Err(e) => anyhow::bail!("create runtime failed: {}", e.to_string()),
        }
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.inner.spawn(future)
    }

    pub fn as_weak(&self) -> WeakRuntime {
        WeakRuntime {
            inner: Rc::downgrade(&self.inner),
        }
    }
}
