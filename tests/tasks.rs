use war_in_tunnels::app::states::skirmish_states::{
    Players,
    tasks::{DiggingTask, MovingTask, Task, Tasks},
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

#[test]
fn movement_progress_is_separate_from_digging_and_resets_for_next_cell() {
    let mut task = MovingTask::new([(5, 3), (5, 4)].into());
    assert_eq!(task.get_movement_progress(), None);
    assert_eq!(task.get_progress(), 0.0);
    assert!(!task.is_finished());
    for _ in 0..46 {
        task.progress();
    }
    assert_eq!(task.get_movement_progress(), Some(50));
    for _ in 46..91 {
        task.progress();
    }
    assert_eq!(task.get_movement_progress(), Some(100));
    assert_eq!(task.get_progress(), 100.0);
    assert!(task.is_finished());
    task.progress();
    assert_eq!(task.get_progress(), 100.0);
    task.pop_front();
    assert_eq!(task.get_movement_progress(), None);
    assert_eq!(task.get_progress(), 0.0);
    assert!(!task.is_finished());
}

#[test]
fn overall_digging_progress_counts_completed_cells_and_current_step() -> () {
    let mut task: DiggingTask = DiggingTask::new([(5, 3), (5, 4)].into(), 25.0);
    assert_eq!(task.get_total_progress(), 0.0);
    task.progress();
    assert_eq!(task.get_progress(), 25.0);
    assert_eq!(task.get_total_progress(), 12.5);
    task.pop_front();
    assert_eq!(task.get_progress(), 0.0);
    assert_eq!(task.get_total_progress(), 50.0);
    task.progress();
    assert_eq!(task.get_total_progress(), 62.5);
    task.pop_front();
    assert_eq!(task.get_total_progress(), 100.0);
    let empty: DiggingTask = DiggingTask::new([].into(), 1.0);
    assert_eq!(empty.get_total_progress(), 100.0);
}

#[test]
fn task_conversion_keeps_completed_steps_and_original_length() -> () {
    let mut digging: DiggingTask = DiggingTask::new([(5, 3), (5, 4)].into(), 25.0);
    digging.pop_front();
    let mut moving: MovingTask = MovingTask::from_digging(digging);
    assert_eq!(moving.get_total_steps(), 2);
    assert_eq!(moving.get_total_progress(), 50.0);
    for _ in 0..91 {
        moving.progress();
    }
    assert_eq!(moving.get_total_progress(), 100.0);
    let mut digging: DiggingTask = DiggingTask::from_moving(moving, 25.0);
    assert_eq!(digging.get_total_steps(), 2);
    assert_eq!(digging.get_total_progress(), 50.0);
    digging.progress();
    assert_eq!(digging.get_total_progress(), 62.5);
}

#[test]
fn digging_queue_formats_step_total_and_pending_progress() -> () {
    let mut miner: MinerUnit = MinerUnit::new(Players::Player, (5, 2));
    let mut task: DiggingTask = DiggingTask::new([(5, 3), (5, 4)].into(), 25.0);
    task.pop_front();
    task.progress();
    miner.set_task(Tasks::Digging(task));
    miner.set_task(Tasks::Moving(MovingTask::new([(5, 5)].into())));
    let lines: Vec<ratatui::text::Line<'_>> = miner.get_tasks_formatted();
    assert!(lines[1].to_string().contains("Step: 25% | Total: 62%"));
    assert!(lines[3].to_string().contains("Step: 0% | Total: 0%"));
}
