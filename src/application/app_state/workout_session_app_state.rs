use crate::{
    application::usecase::workout_session::{
        AddExerciseToWorkoutSession, AddSetToWorkoutSession, CancelWorkoutSession,
        DeleteWorkoutSessionSet, FindCurrentWorkoutSession, FinishWorkoutSession,
        ReadWorkoutSessionHistory, ReadWorkoutSessionWeeklySummary,
        RemoveExerciseFromWorkoutSession, ReorderWorkoutSessionExercises, StartWorkoutSession,
        UpdateWorkoutSessionSet,
    },
    domain::repositories::{
        workout_plan_repository::WorkoutPlanRepository,
        workout_session_repository::WorkoutSessionRepository,
        workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkoutSessionAppState {
    pub start: Arc<StartWorkoutSession>,
    pub find_current: Arc<FindCurrentWorkoutSession>,
    pub finish: Arc<FinishWorkoutSession>,
    pub cancel: Arc<CancelWorkoutSession>,
    pub add_exercise: Arc<AddExerciseToWorkoutSession>,
    pub remove_exercise: Arc<RemoveExerciseFromWorkoutSession>,
    pub reorder_exercises: Arc<ReorderWorkoutSessionExercises>,
    pub add_set: Arc<AddSetToWorkoutSession>,
    pub update_set: Arc<UpdateWorkoutSessionSet>,
    pub delete_set: Arc<DeleteWorkoutSessionSet>,
    pub history: Arc<ReadWorkoutSessionHistory>,
    pub weekly_summary: Arc<ReadWorkoutSessionWeeklySummary>,
}

impl WorkoutSessionAppState {
    pub fn new(
        workout_session_repo: Arc<dyn WorkoutSessionRepository>,
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            start: Arc::new(StartWorkoutSession::new(
                workout_session_repo.clone(),
                workout_plan_repo,
                workout_template_repo,
            )),
            find_current: Arc::new(FindCurrentWorkoutSession::new(workout_session_repo.clone())),
            finish: Arc::new(FinishWorkoutSession::new(workout_session_repo.clone())),
            cancel: Arc::new(CancelWorkoutSession::new(workout_session_repo.clone())),
            add_exercise: Arc::new(AddExerciseToWorkoutSession::new(
                workout_session_repo.clone(),
            )),
            remove_exercise: Arc::new(RemoveExerciseFromWorkoutSession::new(
                workout_session_repo.clone(),
            )),
            reorder_exercises: Arc::new(ReorderWorkoutSessionExercises::new(
                workout_session_repo.clone(),
            )),
            add_set: Arc::new(AddSetToWorkoutSession::new(workout_session_repo.clone())),
            update_set: Arc::new(UpdateWorkoutSessionSet::new(workout_session_repo.clone())),
            delete_set: Arc::new(DeleteWorkoutSessionSet::new(workout_session_repo.clone())),
            history: Arc::new(ReadWorkoutSessionHistory::new(workout_session_repo.clone())),
            weekly_summary: Arc::new(ReadWorkoutSessionWeeklySummary::new(workout_session_repo)),
        }
    }
}
