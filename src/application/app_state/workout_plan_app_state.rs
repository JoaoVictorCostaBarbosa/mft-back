use crate::{
    application::usecase::workout_plan::{
        FindCurrentWorkoutPlan, SetCurrentWorkoutPlan,
        add_workout_template::AddWorkoutTemplateToWorkoutPlan,
        create_workout_plan::CreateWorkoutPlan, delete_workout_plan::DeleteWorkoutPlan,
        find_workout_plan_by_id::FindWorkoutPlanById,
        read_workout_plan_summary::ReadWorkoutPlanSummary,
        remove_workout_template::RemoveWorkoutTemplateFromWorkoutPlan,
        soft_delete_workout_plan::SoftDeleteWorkoutPlan, update_workout_plan::UpdateWorkoutPlan,
    },
    domain::repositories::{
        workout_plan_repository::WorkoutPlanRepository,
        workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkoutPlanAppState {
    pub create: Arc<CreateWorkoutPlan>,
    pub read: Arc<ReadWorkoutPlanSummary>,
    pub update: Arc<UpdateWorkoutPlan>,
    pub delete: Arc<DeleteWorkoutPlan>,
    pub find_by_id: Arc<FindWorkoutPlanById>,
    pub set_current: Arc<SetCurrentWorkoutPlan>,
    pub find_current: Arc<FindCurrentWorkoutPlan>,
    pub soft_delete: Arc<SoftDeleteWorkoutPlan>,
    pub add_workout_template: Arc<AddWorkoutTemplateToWorkoutPlan>,
    pub remove_workout_template: Arc<RemoveWorkoutTemplateFromWorkoutPlan>,
}

impl WorkoutPlanAppState {
    pub fn new(
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            create: Arc::new(CreateWorkoutPlan::new(workout_plan_repo.clone())),
            read: Arc::new(ReadWorkoutPlanSummary::new(workout_plan_repo.clone())),
            update: Arc::new(UpdateWorkoutPlan::new(workout_plan_repo.clone())),
            delete: Arc::new(DeleteWorkoutPlan::new(workout_plan_repo.clone())),
            find_by_id: Arc::new(FindWorkoutPlanById::new(workout_plan_repo.clone())),
            set_current: Arc::new(SetCurrentWorkoutPlan::new(workout_plan_repo.clone())),
            find_current: Arc::new(FindCurrentWorkoutPlan::new(workout_plan_repo.clone())),
            soft_delete: Arc::new(SoftDeleteWorkoutPlan::new(workout_plan_repo.clone())),
            add_workout_template: Arc::new(AddWorkoutTemplateToWorkoutPlan::new(
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            )),
            remove_workout_template: Arc::new(RemoveWorkoutTemplateFromWorkoutPlan::new(
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            )),
        }
    }
}
