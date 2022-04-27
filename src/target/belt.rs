use futures::executor::block_on;
use std::{
    sync::{
        mpsc::{channel, Sender, TryRecvError},
        Arc,
    },
    thread::{self, JoinHandle},
};
use wgpu::{util::StagingBelt, Device, Maintain};

//

pub struct Belt {
    belt: Option<StagingBelt>,
    poll_thread: Option<JoinHandle<()>>,
    poll_stop: Sender<()>,
}

//

impl Belt {
    pub fn new(device: Arc<Device>) -> Self {
        // poll thread
        let (poll_stop, poll_listen) = channel();
        let poll_thread = {
            Some(thread::spawn(move || loop {
                match poll_listen.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }

                device.poll(Maintain::Wait);
                thread::yield_now();
            }))
        };

        let belt = Some(StagingBelt::new(128));

        Self {
            belt,
            poll_thread,
            poll_stop,
        }
    }

    pub fn get(&mut self) -> StagingBelt {
        self.belt
            .take()
            .expect("Cannot start a second frame when the first hasn't been finished yet")
    }

    pub fn set(&mut self, mut belt: StagingBelt) {
        block_on(belt.recall());
        self.belt = Some(belt);
    }
}

impl Drop for Belt {
    fn drop(&mut self) {
        self.poll_stop.send(()).unwrap();
        self.poll_thread
            .take()
            .expect("Engine dropped twice")
            .join()
            .unwrap();
    }
}
