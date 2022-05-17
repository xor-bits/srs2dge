use crate::target::Target;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver},
    Arc,
};
use wgpu::Device;

//

pub struct Catcher {
    error_receiver: Receiver<String>,
    error_listening: Arc<AtomicBool>,
}

//

impl Catcher {
    pub fn new(device: &Device) -> Self {
        // error capturing/handling
        let (error_sender, error_receiver) = channel();
        let listening = Arc::new(AtomicBool::new(false));
        let error_listening = listening.clone();
        device.on_uncaptured_error(move |err| match err {
            wgpu::Error::OutOfMemory { source } => log::error!("Out of memory: {source}"),
            wgpu::Error::Validation {
                source,
                description,
            } => {
                if listening.load(Ordering::SeqCst) {
                    log::warn!("Handled validation error: {source} {description}");
                    error_sender.send(description).unwrap();
                } else {
                    panic!("Unhandled validation error: {source} {description}")
                }
            }
        });

        Self {
            error_receiver,
            error_listening,
        }
    }

    pub fn catch_error<T, F: FnOnce(&Target) -> T>(target: &Target, f: F) -> Result<T, String> {
        let s = &target.catcher;
        s.error_listening.store(true, Ordering::SeqCst);
        let result = f(target);
        s.error_listening.store(false, Ordering::SeqCst);

        if let Ok(err) = s.error_receiver.try_recv() {
            Err(err)
        } else {
            Ok(result)
        }
    }
}
