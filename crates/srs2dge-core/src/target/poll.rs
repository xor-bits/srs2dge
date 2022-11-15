use std::{
    sync::{
        mpsc::{channel, Sender, TryRecvError},
        Arc,
    },
    thread::JoinHandle,
};
use wgpu::{Device, Maintain};

//

#[cfg(not(target_arch = "wasm32"))]
pub struct PollThread {
    poll_thread: Option<JoinHandle<()>>,
    poll_stop: Sender<()>,
}

#[cfg(target_arch = "wasm32")]
pub struct PollThread;

//

#[cfg(not(target_arch = "wasm32"))]
impl PollThread {
    pub fn new(device: Arc<Device>) -> Self {
        let (poll_stop, poll_listen) = channel();

        let poll_thread = Some(std::thread::spawn(move || loop {
            match poll_listen.try_recv() {
                Ok(()) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            device.poll(Maintain::Wait);
        }));

        Self {
            poll_stop,
            poll_thread,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl PollThread {
    pub fn new(_: Arc<Device>) -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for PollThread {
    fn drop(&mut self) {
        self.poll_stop.send(()).unwrap();
        self.poll_thread
            .take()
            .expect("PollThread dropped twice")
            .join()
            .unwrap();
    }
}
