use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::model::DashboardData;

/// A tiny time based cache so that the GUI and every connected OBS browser
/// source share a single upstream request per refresh window.
///
/// The v2 build cached the upstream response for five seconds; we keep the
/// same window.
#[derive(Debug)]
pub struct DashboardCache {
    ttl: Duration,
    inner: Mutex<Option<(Instant, DashboardData)>>,
}

impl Default for DashboardCache {
    fn default() -> Self {
        Self::new(Duration::from_secs(5))
    }
}

impl DashboardCache {
    /// Creates a cache with the given time to live.
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            inner: Mutex::new(None),
        }
    }

    /// Returns the cached payload when it is still fresh.
    pub fn get(&self) -> Option<DashboardData> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        match guard.as_ref() {
            Some((stamp, data)) if stamp.elapsed() < self.ttl => Some(*data),
            _ => None,
        }
    }

    /// Stores a fresh payload.
    pub fn put(&self, data: DashboardData) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        *guard = Some((Instant::now(), data));
    }

    /// Drops the cached payload, e.g. after the configuration changed.
    pub fn invalidate(&self) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Goals;
    use crate::model::RumbleStats;

    fn payload() -> DashboardData {
        DashboardData::new(
            RumbleStats {
                followers: 10,
                subscribers: 5,
            },
            Goals {
                followers: 20,
                subscribers: 10,
            },
        )
    }

    #[test]
    fn an_empty_cache_returns_nothing() {
        let cache = DashboardCache::default();
        assert!(cache.get().is_none());
    }

    #[test]
    fn a_fresh_entry_is_returned() {
        let cache = DashboardCache::default();
        cache.put(payload());

        let cached = cache.get().unwrap();
        assert_eq!(cached.followers, 10);
        assert_eq!(cached.subscribers, 5);
    }

    #[test]
    fn invalidating_drops_the_entry() {
        let cache = DashboardCache::default();
        cache.put(payload());
        cache.invalidate();

        assert!(cache.get().is_none());
    }

    #[test]
    fn a_stale_entry_is_not_returned() {
        let cache = DashboardCache::new(Duration::from_millis(1));
        cache.put(payload());
        std::thread::sleep(Duration::from_millis(10));

        assert!(cache.get().is_none());
    }
}
