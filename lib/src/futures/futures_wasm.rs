use std::{rc::Rc, time::Duration};

use futures::Future;
use tokio::{select, sync::Notify};

use super::JoinHandle;

#[derive(Debug)]
pub struct WasmJoinHandle {
    notify_quit: Rc<Notify>,
}

impl JoinHandle for WasmJoinHandle {
    fn abort(&self) {
        self.notify_quit.notify_one()
    }
}

// tokio's spawn_local returns a JoinHandle, but wasm_bindgen_futures does not, so we can't return
// one here.
pub fn spawn_local(future: impl Future + 'static) -> WasmJoinHandle {
    let join_handle = WasmJoinHandle {
        notify_quit: Default::default(),
    };
    let notify_quit = join_handle.notify_quit.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        select! {
            _ = future => (),
            _ = notify_quit.notified() => (),
        }
    });
    join_handle
}

pub fn sleep(duration: Duration) -> impl Future + 'static {
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut move |resolve, _reject| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                duration.as_millis() as i32,
            )
            .unwrap();
    }))
}
