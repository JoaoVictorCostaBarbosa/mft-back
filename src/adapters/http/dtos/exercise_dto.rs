use crate::adapters::http::dtos::{
    equipment_dto::EquipmentDTO, exercise_type_dto::ExerciseTypeDTO,
    muscle_group_dto::MuscleGroupDTO,
};
use crate::domain::commands::exercise_commands::ExercisePaginationFields;
use crate::domain::entities::pagination::Paginated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PER_PAGE: u32 = 20;
const MAX_PER_PAGE: u32 = 100;

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
pub struct ExercisePaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
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
    pub fn to_pagination_fields(&self) -> ExercisePaginationFields {
        let limit = self
            .per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .clamp(1, MAX_PER_PAGE);
        let page = self.page.unwrap_or(DEFAULT_PAGE).max(1);

        ExercisePaginationFields {
            page,
            per_page: limit,
        }
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
