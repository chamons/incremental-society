use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, is_enum_variant)]
pub enum DelayedAction {
    Edict(String),
    Conversion(String),
    SustainPops(),
    Build(String, usize),
    Destroy(usize, usize),
    Research(String),
    Upgrade(String),
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
        (self.duration as f64 - self.current_tick as f64) / self.duration as f64
    }

    pub fn reset(&mut self) {
        self.current_tick = self.duration;
    }
}

pub fn tick_actions(actions: &mut Vec<Waiter>) -> Vec<DelayedAction> {
    let mut actions_to_remove = vec![];
    let mut actions_to_apply = vec![];
    for (i, a) in actions.iter_mut().enumerate() {
        a.current_tick -= 1;
        if a.current_tick == 0 {
            actions_to_apply.push(a.action.clone());

            if a.reschedule {
                a.current_tick = a.duration;
            } else {
                actions_to_remove.push(i);
            }
        }
    }

    for i in actions_to_remove.iter().rev() {
        actions.remove(*i);
    }
    actions_to_apply
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

    #[test]
    fn reset() {
        let mut x = Waiter::init_one_shot("Test", 200, DelayedAction::SustainPops());
        x.current_tick -= 10;
        x.reset();
        assert_eq!(x.current_tick, x.duration);
    }

    #[test]
    fn tick_actions_none_ready() {
        let mut waiters = vec![
            Waiter::init_one_shot("Test", 10, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test2", 10, DelayedAction::SustainPops()),
        ];

        let fired = tick_actions(&mut waiters);

        assert_eq!(0, fired.len());
        assert_eq!(9, waiters.get(0).unwrap().current_tick);
        assert_eq!(9, waiters.get(1).unwrap().current_tick);
    }

    #[test]
    fn tick_actions_one_repeating_ready() {
        let mut waiters = vec![
            Waiter::init_one_shot("Test", 10, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test2", 10, DelayedAction::SustainPops()),
        ];
        waiters.get_mut(1).unwrap().current_tick = 1;

        let fired = tick_actions(&mut waiters);

        assert_eq!(1, fired.len());
        assert_eq!(9, waiters.get(0).unwrap().current_tick);
        assert_eq!(10, waiters.get(1).unwrap().current_tick);
    }

    #[test]
    fn tick_actions_one_single_shot_ready() {
        let mut waiters = vec![
            Waiter::init_one_shot("Test", 10, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test2", 10, DelayedAction::SustainPops()),
        ];
        waiters.get_mut(0).unwrap().current_tick = 1;
        let fired = tick_actions(&mut waiters);

        assert_eq!(1, fired.len());
        assert_eq!(1, waiters.len());
        assert_eq!(9, waiters.get(0).unwrap().current_tick);
        assert!(waiters.get(0).unwrap().reschedule);
    }

    #[test]
    fn tick_actions_multiple_single_shot_ready() {
        let mut waiters = vec![
            Waiter::init_one_shot("Test", 10, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test2", 5, DelayedAction::SustainPops()),
            Waiter::init_one_shot("Test3", 10, DelayedAction::SustainPops()),
            Waiter::init_one_shot("Test4", 15, DelayedAction::SustainPops()),
        ];
        waiters.get_mut(0).unwrap().current_tick = 1;
        waiters.get_mut(2).unwrap().current_tick = 1;

        let fired = tick_actions(&mut waiters);

        assert_eq!(2, fired.len());
        assert_eq!(2, waiters.len());
        assert_eq!(4, waiters.get(0).unwrap().current_tick);
        assert_eq!(14, waiters.get(1).unwrap().current_tick);
        assert!(waiters.get(0).unwrap().reschedule);
        assert!(!waiters.get(1).unwrap().reschedule);
    }

    #[test]
    fn tick_actions_multiple_both_ready() {
        let mut waiters = vec![
            Waiter::init_one_shot("Test", 5, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test2", 10, DelayedAction::SustainPops()),
            Waiter::init_one_shot("Test3", 10, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test4", 15, DelayedAction::SustainPops()),
            Waiter::init_repeating("Test5", 20, DelayedAction::SustainPops()),
        ];
        waiters.get_mut(1).unwrap().current_tick = 1;
        waiters.get_mut(2).unwrap().current_tick = 1;

        let fired = tick_actions(&mut waiters);

        assert_eq!(2, fired.len());
        assert_eq!(4, waiters.len());
        assert_eq!(4, waiters.get(0).unwrap().current_tick);
        assert!(!waiters.get(0).unwrap().reschedule);

        assert_eq!(10, waiters.get(1).unwrap().current_tick);
        assert!(waiters.get(1).unwrap().reschedule);

        assert_eq!(14, waiters.get(2).unwrap().current_tick);
        assert!(waiters.get(2).unwrap().reschedule);

        assert_eq!(19, waiters.get(3).unwrap().current_tick);
        assert!(waiters.get(3).unwrap().reschedule);
    }
}
