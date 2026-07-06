use crate::application::ports::ExerciseQueries;
use crate::application::usecase::exercise::CreateExercise;
use crate::application::usecase::exercise::DeleteExercise;
use crate::application::usecase::exercise::FindExerciseLastPerformances;
use crate::application::usecase::exercise::GetExerciseById;
use crate::application::usecase::exercise::ReadExercises;
use crate::application::usecase::exercise::ReadPersonalRecords;
use crate::application::usecase::exercise::SearchExercises;
use crate::application::usecase::exercise::SoftDeleteExercise;
use crate::application::usecase::exercise::UpdateExercise;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExerciseAppState {
    pub create: Arc<CreateExercise>,
    pub get_by_id: Arc<GetExerciseById>,
    pub read: Arc<ReadExercises>,
    pub search: Arc<SearchExercises>,
    pub find_last_performances: Arc<FindExerciseLastPerformances>,
    pub read_personal_records: Arc<ReadPersonalRecords>,
    pub update: Arc<UpdateExercise>,
    pub soft_delete: Arc<SoftDeleteExercise>,
    pub delete: Arc<DeleteExercise>,
}

impl ExerciseAppState {
    pub fn new(
        exercise_repo: Arc<dyn ExerciseRepository>,
        exercise_queries: Arc<dyn ExerciseQueries>,
    ) -> Self {
        Self {
            create: Arc::new(CreateExercise::new(exercise_repo.clone())),
            get_by_id: Arc::new(GetExerciseById::new(exercise_repo.clone())),
            read: Arc::new(ReadExercises::new(exercise_repo.clone())),
            search: Arc::new(SearchExercises::new(exercise_repo.clone())),
            find_last_performances: Arc::new(FindExerciseLastPerformances::new(
                exercise_queries.clone(),
            )),
            read_personal_records: Arc::new(ReadPersonalRecords::new(exercise_queries)),
            update: Arc::new(UpdateExercise::new(exercise_repo.clone())),
            soft_delete: Arc::new(SoftDeleteExercise::new(exercise_repo.clone())),
            delete: Arc::new(DeleteExercise::new(exercise_repo.clone())),
        }
    }
}
