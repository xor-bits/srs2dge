use crate::prelude::Lerp;
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

//

#[derive(Debug)]
pub struct Animated<T> {
    init: T,
    frames: Vec<(Duration, AnimationCurve, T)>,
    repeat: AnimationRepeat,
    animator: Arc<Animator>,
    inverted: AtomicBool,
}

#[derive(Debug, Default)]
pub struct Animator {
    state: RwLock<State>,
}

//

impl<T> Animated<T> {
    pub fn new(init: T, animator: Arc<Animator>) -> Self {
        Self {
            init,
            frames: Default::default(),
            repeat: Default::default(),
            animator,
            inverted: Default::default(),
        }
    }

    pub fn then(mut self, duration: Duration, curve: AnimationCurve, content: T) -> Self {
        let time_before = self.total_time();
        self.frames.push((time_before + duration, curve, content));
        self
    }

    pub fn repeat(mut self, repeat: AnimationRepeat) -> Self {
        self.repeat = repeat;
        self
    }

    fn total_time(&self) -> Duration {
        self.frames
            .last()
            .map(|(t, _, _)| *t)
            .unwrap_or(Duration::ZERO)
    }
}

impl<T> Animated<T> {
    pub fn get(&self) -> T
    where
        T: Lerp + Clone,
    {
        let animator = self.animator.state.read().unwrap();
        match animator.deref() {
            State::NotStarted => self.init.clone(),
            State::Running(started) => self.get_at_tp(started.elapsed()),
            State::RunningReverse(started) => self.get_at_tp_rev(started.elapsed()),
            State::Stopped(elapsed) => self.get_at_tp(*elapsed),
            State::StoppedReverse(elapsed) => self.get_at_tp_rev(*elapsed),
        }
    }

    // tp = time point
    fn get_at_tp(&self, mut tp: Duration) -> T
    where
        T: Lerp + Clone,
    {
        if tp >= self.total_time() {
            match self.repeat {
                AnimationRepeat::StopReset => return self.get_last(),
                AnimationRepeat::StopInvert => {
                    self.inverted.store(true, Ordering::SeqCst);
                    return self.get_last();
                }
                AnimationRepeat::ContinueReset => {
                    // no mod(Duration, Duration) in rust
                    let now = tp.as_secs_f64();
                    let total = self.total_time().as_secs_f64();
                    tp = Duration::from_secs_f64(now % total);
                }
                AnimationRepeat::ContinueInvert => {
                    let now = tp.as_secs_f64();
                    let total = self.total_time().as_secs_f64();
                    tp = Duration::from_secs_f64(total - (now % (total * 2.0) - total).abs());
                }
            }
        }

        if self.inverted.load(Ordering::SeqCst) {
            tp = self.total_time() - tp;
        }

        match self.frames.binary_search_by_key(&tp, |(t, _, _)| *t) {
            Ok(v) => self.frames[v].2.clone(),
            Err(middle) => {
                let (left_frame_tp, left_frame) = if middle == 0 {
                    (Duration::ZERO, &self.init)
                } else {
                    let (a, _, b) = &self.frames[middle - 1];
                    (*a, b)
                };
                let (right_frame_tp, curve, right_frame) = self
                    .frames
                    .get(middle)
                    .or_else(|| self.frames.last())
                    .map(|(tp, curve, v)| (*tp, curve, v))
                    .unwrap_or((Duration::ZERO, &AnimationCurve::Instant, &self.init));

                let mut i = (tp - left_frame_tp).as_secs_f32()
                    / (right_frame_tp - left_frame_tp).as_secs_f32();
                if !(0.0..=1.0).contains(&i) {
                    i = 1.0;
                }

                i = curve.run(i);

                left_frame.get(right_frame, i)
            }
        }
    }

    fn get_at_tp_rev(&self, tp: Duration) -> T
    where
        T: Lerp + Clone,
    {
        self.get_at_tp(self.total_time().saturating_sub(tp))
    }

    fn get_last(&self) -> T
    where
        T: Clone,
    {
        self.frames
            .last()
            .map(|(_, _, val)| val.clone())
            .unwrap_or_else(|| self.init.clone())
    }
}

impl<T> Clone for Animated<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            init: self.init.clone(),
            frames: self.frames.clone(),
            repeat: self.repeat,
            animator: self.animator.clone(),
            inverted: self.inverted.load(Ordering::SeqCst).into(),
        }
    }
}

//

impl Animator {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: RwLock::new(State::NotStarted),
        })
    }

    /// triggers the animator or keeps it running
    pub fn trigger(&self) {
        let mut lock = self.state.write().unwrap();
        match lock.deref() {
            State::NotStarted
            | State::RunningReverse(_)
            | State::Stopped(_)
            | State::StoppedReverse(_) => *lock = State::Running(Instant::now()),
            State::Running(_) => {}
        };
    }

    /// triggers or resets the animator
    pub fn re_trigger(&self) {
        *self.state.write().unwrap() = State::Running(Instant::now());
    }

    /// triggers the animator or keeps it running
    pub fn trigger_rev(&self) {
        let mut lock = self.state.write().unwrap();
        match lock.deref() {
            State::NotStarted
            | State::Running(_)
            | State::Stopped(_)
            | State::StoppedReverse(_) => *lock = State::RunningReverse(Instant::now()),
            State::RunningReverse(_) => {}
        };
    }

    /// triggers or resets the animator
    pub fn re_trigger_rev(&self) {
        *self.state.write().unwrap() = State::RunningReverse(Instant::now());
    }

    /// stops the animator
    pub fn stop(&self) {
        let mut lock = self.state.write().unwrap();
        *lock = match lock.deref() {
            State::NotStarted => State::NotStarted,
            State::Running(r) => State::Stopped(r.elapsed()),
            State::RunningReverse(r) => State::StoppedReverse(r.elapsed()),
            State::Stopped(s) => State::Stopped(*s),
            State::StoppedReverse(s) => State::StoppedReverse(*s),
        };
    }

    /// stops and resets the animator
    pub fn reset(&self) {
        *self.state.write().unwrap() = State::NotStarted;
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationCurve {
    Instant,

    /// y = x
    #[default]
    Linear,

    /// - 2*x^3 + 3*x^2
    Poly,

    /// 1/2 + sin(pi*(x-1/2))/2
    Sine,

    /// y = 1/e * x*e^x
    Exponential,
}

impl AnimationCurve {
    pub fn run(self, x: f32) -> f32 {
        use std::f32::consts::*;
        match self {
            AnimationCurve::Instant => 0.0,
            AnimationCurve::Linear => x,
            AnimationCurve::Poly => -2.0 * x.powi(3) + 3.0 * x.powi(2),
            AnimationCurve::Sine => 0.5 * (1.0 + (PI * (x - 0.5)).sin()),
            AnimationCurve::Exponential => 1.0 / E * x * x.exp(),
        }
    }
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationRepeat {
    /// Reset back to frame 0 and stop
    #[default]
    StopReset,

    /// Set direction to backwards and stop
    StopInvert,

    /// Reset back to frame 0 and continue
    ContinueReset,

    /// Set direction to backwards and continue
    ContinueInvert,
}

//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum State {
    #[default]
    NotStarted,
    Running(Instant),
    RunningReverse(Instant),
    Stopped(Duration),
    StoppedReverse(Duration),
}

//

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_curves() {
        use AnimationCurve::*;
        for curve in [Instant, Linear, Poly, Sine, Exponential] {
            for i in 0..=100 {
                let x = i as f32 * 0.01;
                let y = curve.run(x);

                assert!((0.0..=1.0).contains(&x));
                assert!((0.0..=1.0).contains(&y));
            }
        }
    }

    #[test]
    fn test_interpolate() {
        let dur = Duration::from_millis(50);

        let animator = Animator::new();
        let animation = Animated::new(0.0, animator.clone())
            .then(dur, AnimationCurve::Linear, 1.0)
            .then(dur, AnimationCurve::Linear, 0.0)
            .then(dur, AnimationCurve::Linear, 1.0);
        animator.trigger();

        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());

        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
    }

    #[test]
    fn test_repeat_a() {
        let dur = Duration::from_millis(50);

        let animator = Animator::new();
        let animation = Animated::new(0.0, animator.clone())
            .then(dur, AnimationCurve::Linear, 1.0)
            .then(dur, AnimationCurve::Linear, 0.0)
            .repeat(AnimationRepeat::ContinueReset);
        animator.trigger();

        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
    }

    #[test]
    fn test_repeat_b() {
        let dur = Duration::from_millis(50);

        let animator = Animator::new();
        let animation = Animated::new(0.0, animator.clone())
            .then(dur, AnimationCurve::Linear, 1.0)
            .repeat(AnimationRepeat::ContinueInvert);
        animator.trigger();

        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() >= 0.5, "{}", animation.get());
        thread::sleep(dur);
        assert!(animation.get() <= 0.5, "{}", animation.get());
    }
}
