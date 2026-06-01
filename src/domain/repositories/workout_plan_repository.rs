use crate::domain::{
    commands::workout_plan_command::WorkoutPlanFilterFields,
    entities::workout_plan::{WorkoutPlan, WorkoutPlanSummary},
    errors::domain_error::DomainError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait WorkoutPlanRepository: Send + Sync {
    async fn save(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError>;
    async fn read_summary(
        &self,
        filter: WorkoutPlanFilterFields,
    ) -> Result<Vec<WorkoutPlanSummary>, DomainError>;
    async fn find_by_id(&self, workout_plan_id: Uuid) -> Result<WorkoutPlan, DomainError>;
    async fn update(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError>;
    async fn delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError>;
    async fn soft_delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError>;
    async fn add_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn remove_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError>;
}
