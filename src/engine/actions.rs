use crate::actions::{DelayedAction, Waiter};

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
    use crate::actions::DelayedAction;

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
