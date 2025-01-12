use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovRateLimiter,
};
use std::num::NonZeroU32;
use tokio::time::Duration;

/// Our custom RateLimiter struct that wraps governor's RateLimiter
pub struct RateLimiter {
    // Note the full generic signature in 0.8:
    // RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>
    inner: GovRateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
}

impl RateLimiter {
    /// Create a new RateLimiter with a specified steady rate and an optional burst capacity.
    ///
    /// - `requests_per_second`: your desired steady rate
    /// - `burst_size`: how many extra tokens you can "burst" above that rate
    pub fn new(requests_per_second: u32, burst_size: u32) -> Self {
        // We'll allow up to `requests_per_second` tokens each second, plus
        // a short burst of up to `burst_size` additional tokens at once.
        let quota = Quota::with_period(Duration::from_secs(1))
            .unwrap()
            // .allow_burst(...) sets how many tokens we can accumulate
            .allow_burst(NonZeroU32::new(burst_size).unwrap())
            // .allow_burst(...) sets our steady-state tokens per period and burst size
            .allow_burst(NonZeroU32::new(requests_per_second + burst_size).unwrap());

        // .direct(...) creates a limiter with NotKeyed + InMemoryState + DefaultClock + NoOpMiddleware
        let limiter = GovRateLimiter::direct(quota);
        Self { inner: limiter }
    }

    /// Acquire 1 permit, asynchronously blocking until available.
    pub async fn acquire(&self) {
        self.inner.until_ready().await;
    }
}
