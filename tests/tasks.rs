use war_in_tunnels::app::states::skirmish_states::{
    Players,
    tasks::{DiggingTask, Task, Tasks},
    units::{MinerUnit, Unit},
};

#[test]
fn digging_progress_accumulates_and_stops_at_completion() {
    let mut task = DiggingTask::new([(5, 3)].into(), 40.0);
    assert!(!task.is_finished());
    task.progress();
    assert_eq!(task.get_progress(), 40.0);
    assert!(!task.is_finished());
    for _ in 0..3 {
        task.progress();
    }
    assert_eq!(task.get_progress(), 100.0);
    assert!(task.is_finished());
}

#[test]
fn advancing_path_resets_progress_and_handles_empty_path() {
    let mut task = DiggingTask::new([(5, 3), (5, 4)].into(), 25.0);
    task.progress();
    task.pop_front();
    assert_eq!(task.current_target(), Some((5, 4)));
    assert_eq!(task.get_progress(), 0.0);
    task.pop_front();
    task.pop_front();
    assert_eq!(task.current_target(), None);
    assert!(task.get_path().is_empty());
}

#[test]
fn miners_keep_tasks_in_submission_order() {
    let mut miner = MinerUnit::new(Players::Player, (5, 2));
    for target in [(5, 3), (6, 3)] {
        miner.set_task(Tasks::Digging(DiggingTask::new([target].into(), 1.0)));
    }
    assert_eq!(miner.get_ref_tasks().len(), 2);
    assert_eq!(
        *miner
            .get_mut_tasks()
            .pop_front()
            .unwrap()
            .get_path_front_coords(),
        (5, 3)
    );
    assert_eq!(*miner.get_ref_tasks()[0].get_path_front_coords(), (6, 3));
}
