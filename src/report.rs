use std::{
    ops::Deref,
    time::{Duration, Instant},
};

pub struct Reporter {
    count: u32,
    elapsed: Duration,
    report_timer: Instant,
    report_interval: Duration,

    last_frametime: Option<Duration>,
    last_fps: Option<f64>,
}

pub struct Timer {
    begin: Instant,
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new_with_interval(Duration::from_secs(3))
    }
}

impl Reporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_interval(report_interval: Duration) -> Self {
        Self {
            count: 0_u32,
            elapsed: Duration::default(),
            report_timer: Instant::now(),
            report_interval,

            last_frametime: None,
            last_fps: None,
        }
    }

    pub fn begin(&self) -> Timer {
        Timer {
            begin: Instant::now(),
        }
    }

    pub fn end(&mut self, timer: Timer) {
        self.elapsed += timer.begin.elapsed();
        self.count += 1;

        if self.report_timer.elapsed() >= self.report_interval {
            let avg = self.elapsed / self.count;
            let fps = self.count as f64 / self.report_interval.as_secs_f64();

            log::debug!(
				"Report ({}s) \nAVG frametime: {:.2?}\nAVG FPS: {:.2} (based on avg frametime)\nREAL FPS: {:.2}",
				self.report_interval.as_secs_f64(),
				avg,
				1.0 / avg.as_secs_f64(),
				fps
			);

            self.count = 0;
            self.elapsed = Duration::default();
            self.report_timer = Instant::now();
            self.last_frametime = Some(avg);
            self.last_fps = Some(fps);
        }
    }

    pub fn should_report(&self) -> bool {
        self.report_timer.elapsed() >= self.report_interval
    }

    pub fn report(&mut self) {
        let avg = self.elapsed / self.count;
        log::debug!(
			"Report ({}s) \nAVG frametime: {:.2?}\nAVG FPS: {:.2} (based on avg frametime)\nREAL FPS: {:.2}",
			self.report_interval.as_secs_f64(),
			avg,
			self.report_interval.as_secs_f64() / avg.as_secs_f64(),
			self.count as f64 / self.report_interval.as_secs_f64()
		);
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.elapsed = Duration::default();
        self.report_timer = Instant::now();
    }

    pub fn report_maybe(&mut self) {
        if self.should_report() {
            self.report();
            self.reset();
        }
    }

    pub fn last(&self) -> Option<(Duration, f64)> {
        Some((self.last_frametime?, self.last_fps?))
    }

    pub fn last_string(&self) -> (String, String) {
        (
            self.last_frametime
                .map(|ft| format!("{:?}", ft))
                .unwrap_or_else(|| "...".into()),
            self.last_fps
                .map(|fps| format!("{}", fps))
                .unwrap_or_else(|| "...".into()),
        )
    }
}

impl Deref for Timer {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.begin
    }
}
