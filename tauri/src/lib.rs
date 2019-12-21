#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate web_view;

pub mod command;
pub mod dir;
pub mod event;
pub mod file;
pub mod file_system;
pub mod rpc;
pub mod salt;

pub mod version;

use web_view::*;

use threadpool::ThreadPool;

thread_local!(static POOL: ThreadPool = ThreadPool::new(4));

pub fn spawn<F: FnOnce() -> () + Send + 'static>(what: F) {
  POOL.with(|thread| {
    thread.execute(move || {
      what();
    });
  });
}

pub fn execute_promise<T: 'static, F: FnOnce() -> Result<String, String> + Send + 'static>(
  webview: &mut WebView<'_, T>,
  what: F,
  callback: String,
  error: String,
) {
  let handle = webview.handle();
  POOL.with(|thread| {
    thread.execute(move || {
      let callback_string = rpc::format_callback_result(what(), callback, error);
      handle
        .dispatch(move |_webview| _webview.eval(callback_string.as_str()))
        .unwrap()
    });
  });
}
