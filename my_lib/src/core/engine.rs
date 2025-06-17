use crate::core::engine_config::EngineConfig;
use crate::core::engine_context::EngineContext;
use crate::core::engine_error::EngineError;
use crate::core::models::user_info::UserInfo;
use crate::core::network::network;
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use threadpool::ThreadPool;
use tokio::sync::oneshot;

/// 回调线程池，最大线程数
const CB_THREAD_POOL_MAX_NUM: usize = 3;

pub struct Engine {
    ctx: Arc<EngineContext>,
    cb_pool: ThreadPool,
    tx: Sender<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Engine {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                let mut futures = rx.iter();
                while let Some(future) = futures.next() {
                    future.await;
                }
            });
        });
        let ctx = Arc::new(EngineContext::new(config));
        Engine {
            tx,
            ctx,
            cb_pool: ThreadPool::new(CB_THREAD_POOL_MAX_NUM),
        }
    }
}

/// 基本数据类型
impl Engine {
    pub fn add_int(&self, left: i32, right: i32) -> i32 {
        left + right
    }

    pub fn add_string(&self, left: &str, right: &str) -> String {
        format!("{}{}", left, right)
    }

    pub fn add_double(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    pub fn add_bool(&self, left: bool, right: bool) -> bool {
        left && right
    }
}

/// 复杂数据类型
impl Engine {
    pub fn update_user_info<CB>(&self, user_info: UserInfo, cb: CB)
    where
        CB: FnOnce(Result<UserInfo, EngineError>) + Send + 'static,
    {
        let user_info = UserInfo::new(user_info.get_id(), "new_name", 123);
        self.post(async move { cb(Ok(user_info)) });
    }
}

/// 函数调用
impl Engine {
    pub fn get_url<CB>(&self, url: &str, cb: CB)
    where
        CB: FnOnce(Result<String, EngineError>) + Send + 'static,
    {
        let url_string = url.to_string();
        let ctx_clone = self.ctx.clone();
        self.post(async move {
            let ret = network::get_url(&ctx_clone, &url_string).await;
            cb(ret);
        });
    }
}

impl Engine {
    /// async 转同步请求
    fn invoke<T, F>(&self, future: T) -> F
    where
        T: Future<Output = F> + Send + 'static,
        F: Send + 'static,
    {
        let (result_tx, result_rx) = oneshot::channel();
        let invoke_future = async move {
            let result = future.await;
            let _ = result_tx.send(result);
        };
        self.tx.send(Box::pin(invoke_future)).unwrap();
        result_rx.blocking_recv().unwrap()
    }

    /// async 转异步请求
    fn post<T>(&self, future: T)
    where
        T: Future<Output = ()> + Send + 'static,
    {
        self.tx.send(Box::pin(future)).unwrap();
    }

    /// 为避免回调中的耗时操作阻塞，以及回调中调用其他接口造成死锁，所有回调必须放入线程池执行
    fn cb_pool_once<F, R>(&self, cb: F) -> impl FnOnce(R)
    where
        F: FnOnce(R) + Send + 'static,
        R: Send + 'static,
    {
        let cb_pool = self.cb_pool.clone();
        move |r1| cb_pool.execute(move || cb(r1))
    }

    /// cb_pool 重载接口，接受两个回调参数
    fn cb_pool_once2<F, R1, R2>(&self, cb: F) -> impl FnOnce(R1, R2)
    where
        F: FnOnce(R1, R2) + Send + 'static,
        R1: Send + 'static,
        R2: Send + 'static,
    {
        let cb_pool = self.cb_pool.clone();
        move |r1, r2| cb_pool.execute(move || cb(r1, r2))
    }

    /// cb_pool 重载接口，接受两个回调参数，返回 Fn(R1, R2)
    fn cb_pool_fn2<F, R1, R2>(&self, cb: F) -> impl Fn(R1, R2)
    where
        F: Fn(R1, R2) + Clone + Send + 'static,
        R1: Clone + Send + 'static,
        R2: Clone + Send + 'static,
    {
        let cb_pool = self.cb_pool.clone();
        move |r1, r2| {
            let cb_clone = cb.clone();
            let r1_clone = r1.clone();
            let r2_clone = r2.clone();
            cb_pool.execute(move || cb_clone(r1_clone, r2_clone))
        }
    }

    /// cb_pool 重载接口，接受三个回调参数，返回 Fn(R1, R2, R3)
    fn cb_pool_fn3<F, R1, R2, R3>(&self, cb: F) -> impl Fn(R1, R2, R3)
    where
        F: Fn(R1, R2, R3) + Clone + Send + 'static,
        R1: Clone + Send + 'static,
        R2: Clone + Send + 'static,
        R3: Clone + Send + 'static,
    {
        let cb_pool = self.cb_pool.clone();
        move |r1, r2, r3| {
            let cb_clone = cb.clone();
            let r1_clone = r1.clone();
            let r2_clone = r2.clone();
            let r3_clone = r3.clone();
            cb_pool.execute(move || cb_clone(r1_clone, r2_clone, r3_clone))
        }
    }
}
