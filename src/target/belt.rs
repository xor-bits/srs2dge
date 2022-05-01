use main_game_loop::as_async;
use std::sync::Arc;
use wgpu::{util::StagingBelt, Device};

#[cfg(not(target_arch = "wasm32"))]
use {
    std::{
        sync::mpsc::{channel, Sender, TryRecvError},
        thread::JoinHandle,
    },
    wgpu::Maintain,
};

//

pub struct Belt {
    belt: Option<StagingBelt>,

    #[cfg(not(target_arch = "wasm32"))]
    _poll: PollThread,
}

#[cfg(not(target_arch = "wasm32"))]
struct PollThread {
    poll_thread: Option<JoinHandle<()>>,
    poll_stop: Sender<()>,
}

//

impl Belt {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(device: Arc<Device>) -> Self {
        let belt = Some(StagingBelt::new(128));
        let _poll = PollThread::new(device);

        Self { belt, _poll }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(_: Arc<Device>) -> Self {
        let belt = Some(StagingBelt::new(128));

        Self { belt }
    }

    pub fn get(&mut self) -> StagingBelt {
        self.belt
            .take()
            .expect("Cannot start a second frame when the first hasn't been finished yet")
    }

    pub fn set(&mut self, mut belt: StagingBelt) {
        as_async(belt.recall());
        self.belt = Some(belt);
    }
}

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
            #[cfg(target_arch = "wasm32")]
            thread::yield_now();
        }));

        Self {
            poll_stop,
            poll_thread,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for PollThread {
    fn drop(&mut self) {
        self.poll_stop.send(()).unwrap();
        self.poll_thread
            .take()
            .expect("Engine dropped twice")
            .join()
            .unwrap();
    }
}
