use crate::application::usecase::workout_plan::AddWorkoutTemplateToWorkoutPlan;
use crate::application::usecase::workout_plan::CreateWorkoutPlan;
use crate::application::usecase::workout_plan::DeleteWorkoutPlan;
use crate::application::usecase::workout_plan::FindCurrentWorkoutPlan;
use crate::application::usecase::workout_plan::FindNextRoutineItem;
use crate::application::usecase::workout_plan::FindWorkoutPlanById;
use crate::application::usecase::workout_plan::ReadWorkoutPlanSummary;
use crate::application::usecase::workout_plan::RemoveRoutineItem;
use crate::application::usecase::workout_plan::RemoveWorkoutTemplateFromWorkoutPlan;
use crate::application::usecase::workout_plan::SetCurrentWorkoutPlan;
use crate::application::usecase::workout_plan::SoftDeleteWorkoutPlan;
use crate::application::usecase::workout_plan::UpdateRoutineItem;
use crate::application::usecase::workout_plan::UpdateWorkoutPlan;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
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
    pub update_routine_item: Arc<UpdateRoutineItem>,
    pub remove_routine_item: Arc<RemoveRoutineItem>,
    pub find_next_routine_item: Arc<FindNextRoutineItem>,
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
            update_routine_item: Arc::new(UpdateRoutineItem::new(
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            )),
            remove_routine_item: Arc::new(RemoveRoutineItem::new(workout_plan_repo.clone())),
            find_next_routine_item: Arc::new(FindNextRoutineItem::new(workout_plan_repo.clone())),
        }
    }
}
