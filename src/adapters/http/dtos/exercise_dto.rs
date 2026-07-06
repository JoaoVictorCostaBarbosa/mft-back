use crate::adapters::http::dtos::EquipmentDTO;
use crate::adapters::http::dtos::ExerciseTypeDTO;
use crate::adapters::http::dtos::MuscleGroupDTO;
use crate::adapters::http::dtos::SetTypeDTO;
use crate::application::pagination::PageRequest;
use crate::domain::entities::Paginated;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExerciseRequest {
    pub name: String,
    pub exercise_type: ExerciseTypeDTO,
    pub equipment: EquipmentDTO,
    pub muscle_group: MuscleGroupDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExerciseResponseDTO {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: ExerciseTypeDTO,
    pub equipment: EquipmentDTO,
    pub muscle_group: MuscleGroupDTO,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExerciseUpdateRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub exercise_type: Option<ExerciseTypeDTO>,
    pub equipment: Option<EquipmentDTO>,
    pub muscle_group: Option<MuscleGroupDTO>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExerciseLastPerformancesRequestDTO {
    pub exercise_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExerciseLastPerformancesResponseDTO {
    pub items: Vec<ExerciseLastPerformanceItemDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExerciseLastPerformanceItemDTO {
    pub exercise_id: Uuid,
    pub last_session_id: Uuid,
    pub performed_at: DateTime<Utc>,
    pub sets: Vec<ExerciseLastPerformanceSetDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExerciseLastPerformanceSetDTO {
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
    pub order: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExercisePersonalRecordDTO {
    pub exercise_id: Uuid,
    pub exercise_name: String,
    pub max_weight: f32,
    pub reps: u32,
    pub achieved_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExercisePersonalRecordsResponseDTO {
    pub items: Vec<ExercisePersonalRecordDTO>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExercisePaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExercisePaginatedResponseDTO {
    pub items: Vec<ExerciseResponseDTO>,
    pub pagination: ExercisePaginationMetaDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExercisePaginationMetaDTO {
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u32,
    #[serde(rename = "currentPage")]
    pub current_page: u32,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    #[serde(rename = "hasPrevious")]
    pub has_previous: bool,
}

impl ExercisePaginationQuery {
    pub fn to_page_request(&self) -> PageRequest {
        PageRequest::new(self.page, self.per_page)
    }
}

impl ExercisePaginatedResponseDTO {
    pub fn new(page: Paginated<ExerciseResponseDTO>) -> Self {
        Self {
            pagination: ExercisePaginationMetaDTO {
                total_items: page.total_items,
                items_per_page: page.items_per_page,
                current_page: page.current_page,
                total_pages: page.total_pages(),
                has_next: page.has_next(),
                has_previous: page.has_previous(),
            },
            items: page.items,
        }
    }
}
