use crate::domain::commands::WorkoutPlanFilterFields;
use crate::domain::entities::WorkoutPlan;
use crate::domain::entities::WorkoutPlanRoutineItem;
use crate::domain::entities::WorkoutPlanSummary;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait WorkoutPlanRepository: Send + Sync {
    async fn save(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError>;
    async fn read_summary(
        &self,
        filter: WorkoutPlanFilterFields,
    ) -> Result<Vec<WorkoutPlanSummary>, DomainError>;
    async fn find_by_id(&self, workout_plan_id: Uuid) -> Result<WorkoutPlan, DomainError>;
    async fn find_current_user_plan(&self, user_id: Uuid) -> Result<WorkoutPlan, DomainError>;
    async fn set_current(&self, user_id: Uuid, wp_id: Uuid) -> Result<(), DomainError>;
    async fn update(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError>;
    async fn delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError>;
    async fn soft_delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError>;
    async fn add_routine_item(
        &self,
        routine_item: &WorkoutPlanRoutineItem,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn update_routine_item(
        &self,
        routine_item: &WorkoutPlanRoutineItem,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn remove_routine_item(
        &self,
        workout_plan_id: Uuid,
        routine_item_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn remove_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError>;
}
