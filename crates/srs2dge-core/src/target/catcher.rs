use crate::target::Target;
use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver},
        Arc,
    },
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
            wgpu::Error::OutOfMemory { source } => tracing::error!("Out of memory: {source}"),
            wgpu::Error::Validation {
                source,
                description,
            } => {
                if listening.load(Ordering::SeqCst) {
                    tracing::warn!("Handled validation error: {source} {description}");
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

    /// run something while listening for wgpu errors
    pub fn catch_error<T, F: FnOnce(&Target) -> T>(target: &Target, f: F) -> Result<T, String> {
        let s = &target.catcher;

        // clear the error receiver
        while let Ok(_) = s.error_receiver.try_recv() {}

        // start listening for errors and run the func
        s.error_listening.store(true, Ordering::SeqCst);
        let result = f(target);
        s.error_listening.store(false, Ordering::SeqCst);

        // return the error
        if let Ok(err) = s.error_receiver.try_recv() {
            Err(err)
        } else {
            Ok(result)
        }
    }

    /// run something and await on it while listening for wgpu errors
    pub async fn catch_error_async<T, Fut, F>(target: &Target, f: F) -> Result<T, String>
    where
        F: FnOnce(&Target) -> Fut,
        Fut: Future<Output = T>,
    {
        let s = &target.catcher;

        // clear the error receiver
        while let Ok(_) = s.error_receiver.try_recv() {}

        // start listening for errors and run the func
        s.error_listening.store(true, Ordering::SeqCst);
        let result = f(target).await;
        s.error_listening.store(false, Ordering::SeqCst);

        // return the error
        if let Ok(err) = s.error_receiver.try_recv() {
            Err(err)
        } else {
            Ok(result)
        }
    }
}
