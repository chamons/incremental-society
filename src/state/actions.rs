use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelayedAction {
    Edict(String),
    Conversion(String),
    SustainPops(),
    Build(String, usize),
    Destroy(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waiter {
    pub name: String,
    pub current_tick: u32,
    pub duration: u32,
    pub action: DelayedAction,
    pub reschedule: bool,
}

impl Waiter {
    pub fn init_one_shot(name: &str, duration: u32, action: DelayedAction) -> Waiter {
        Waiter {
            name: name.to_string(),
            current_tick: duration,
            duration,
            action,
            reschedule: false,
        }
    }

    pub fn init_repeating(name: &str, duration: u32, action: DelayedAction) -> Waiter {
        Waiter {
            name: name.to_string(),
            current_tick: duration,
            duration,
            action,
            reschedule: true,
        }
    }

    pub fn percentage(&self) -> f64 {
        ((self.duration as f64 - self.current_tick as f64) / self.duration as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn waiter_percentage() {
        let mut x = Waiter::init_one_shot("Test", 200, DelayedAction::SustainPops());
        assert_approx_eq!(0.0, x.percentage());

        x.current_tick = 180;
        assert_approx_eq!(0.1, x.percentage());
        x.current_tick = 150;
        assert_approx_eq!(0.25, x.percentage());
        x.current_tick = 16;
        assert_approx_eq!(0.92, x.percentage());
        x.current_tick = 2;
        assert_approx_eq!(0.99, x.percentage());
    }
}
