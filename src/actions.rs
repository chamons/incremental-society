use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelayedAction {
    Edict(String),
    Conversion(String),
    SustainPops(),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waiter {
    pub current_tick: u32,
    pub duration: u32,
    pub action: DelayedAction,
    pub reschedule: bool,
}

impl Waiter {
    pub fn init_one_shot(duration: u32, action: DelayedAction) -> Waiter {
        Waiter {
            current_tick: duration,
            duration,
            action,
            reschedule: false,
        }
    }

    pub fn init_repeating(duration: u32, action: DelayedAction) -> Waiter {
        Waiter {
            current_tick: duration,
            duration,
            action,
            reschedule: true,
        }
    }

    pub fn percentage(&self) -> f64 {
        ((self.duration as f64 - self.current_tick as f64) / self.duration as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn waiter_percentage() {
        let mut x = Waiter::init_one_shot(200, DelayedAction::SustainPops());
        assert_approx_eq!(0.0, x.percentage());

        x.current_tick = 180;
        assert_approx_eq!(10.0, x.percentage());
        x.current_tick = 150;
        assert_approx_eq!(25.0, x.percentage());
        x.current_tick = 16;
        assert_approx_eq!(92.0, x.percentage());
        x.current_tick = 2;
        assert_approx_eq!(99.0, x.percentage());
    }
}
