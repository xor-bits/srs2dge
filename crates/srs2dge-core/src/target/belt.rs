use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use wgpu::util::StagingBelt;

//

pub struct BeltPool {
    belts: Receiver<StagingBelt>,
    returns: SyncSender<StagingBelt>,
}

//

impl BeltPool {
    pub fn new() -> Self {
        let (returns, belts) = sync_channel(8);

        Self { belts, returns }
    }

    pub fn recv(&self) -> StagingBelt {
        if let Ok(belt) = self.belts.try_recv() {
            return belt;
        };

        tracing::info!("Creating a new StagingBelt");

        StagingBelt::new(128)
    }

    pub fn send(&self, mut belt: StagingBelt) {
        belt.recall();
        self.returns.send(belt).unwrap();
    }
}
