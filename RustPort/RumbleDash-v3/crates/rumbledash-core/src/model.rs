use serde::{Deserialize, Serialize};

use crate::config::Goals;

/// Raw counters returned by the Rumble livestream API.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RumbleStats {
    /// Current follower count.
    pub followers: u64,
    /// Current subscriber count.
    pub subscribers: u64,
}

/// The payload served by `/api/dashboard` and rendered by the GUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardData {
    /// Current follower count.
    pub followers: u64,
    /// Current subscriber count.
    pub subscribers: u64,
    /// The configured goals.
    pub goals: Goals,
}

impl DashboardData {
    /// Builds the payload from raw stats plus the active goals.
    pub fn new(stats: RumbleStats, goals: Goals) -> Self {
        Self {
            followers: stats.followers,
            subscribers: stats.subscribers,
            goals,
        }
    }

    /// Completion of the follower goal, clamped to `0.0..=1.0`.
    pub fn follower_progress(&self) -> f32 {
        progress(self.followers, self.goals.followers)
    }

    /// Completion of the subscriber goal, clamped to `0.0..=1.0`.
    pub fn subscriber_progress(&self) -> f32 {
        progress(self.subscribers, self.goals.subscribers)
    }
}

/// Ratio of `current` to `goal`, guarding against a zero goal.
fn progress(current: u64, goal: u64) -> f32 {
    if goal == 0 {
        0.0
    } else {
        (current as f32 / goal as f32).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data(followers: u64, subscribers: u64, goals: Goals) -> DashboardData {
        DashboardData::new(
            RumbleStats {
                followers,
                subscribers,
            },
            goals,
        )
    }

    #[test]
    fn progress_is_the_ratio_of_current_to_goal() {
        let data = data(
            25,
            5,
            Goals {
                followers: 50,
                subscribers: 10,
            },
        );

        assert!((data.follower_progress() - 0.5).abs() < f32::EPSILON);
        assert!((data.subscriber_progress() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn progress_is_clamped_to_one() {
        let data = data(
            100,
            100,
            Goals {
                followers: 50,
                subscribers: 10,
            },
        );

        assert_eq!(data.follower_progress(), 1.0);
        assert_eq!(data.subscriber_progress(), 1.0);
    }

    #[test]
    fn a_zero_goal_does_not_divide_by_zero() {
        let data = data(
            10,
            10,
            Goals {
                followers: 0,
                subscribers: 0,
            },
        );

        assert_eq!(data.follower_progress(), 0.0);
        assert_eq!(data.subscriber_progress(), 0.0);
    }
}
