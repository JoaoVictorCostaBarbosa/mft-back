use crate::application::usecase::workout_template::AddExerciseToWorkoutTemplate;
use crate::application::usecase::workout_template::CreateWorkoutTemplate;
use crate::application::usecase::workout_template::DeleteWorkoutTemplate;
use crate::application::usecase::workout_template::FindWorkoutTemplateById;
use crate::application::usecase::workout_template::ReadWorkoutsTemplate;
use crate::application::usecase::workout_template::RemoveExerciseFromWorkoutTemplate;
use crate::application::usecase::workout_template::SoftDeleteWorkoutTemplate;
use crate::application::usecase::workout_template::UpdateWorkoutTemplate;
use crate::domain::repositories::ExerciseRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct WorkoutTemplateAppState {
    pub create: Arc<CreateWorkoutTemplate>,
    pub read: Arc<ReadWorkoutsTemplate>,
    pub update: Arc<UpdateWorkoutTemplate>,
    pub delete: Arc<DeleteWorkoutTemplate>,
    pub find_by_id: Arc<FindWorkoutTemplateById>,
    pub soft_delete: Arc<SoftDeleteWorkoutTemplate>,
    pub add_exercise: Arc<AddExerciseToWorkoutTemplate>,
    pub remove_exercise: Arc<RemoveExerciseFromWorkoutTemplate>,
}

impl WorkoutTemplateAppState {
    pub fn new(
        workout_repo: Arc<dyn WorkoutTemplateRepository>,
        exercise_repo: Arc<dyn ExerciseRepository>,
    ) -> Self {
        Self {
            create: Arc::new(CreateWorkoutTemplate::new(workout_repo.clone())),
            read: Arc::new(ReadWorkoutsTemplate::new(workout_repo.clone())),
            update: Arc::new(UpdateWorkoutTemplate::new(workout_repo.clone())),
            delete: Arc::new(DeleteWorkoutTemplate::new(workout_repo.clone())),
            find_by_id: Arc::new(FindWorkoutTemplateById::new(workout_repo.clone())),
            soft_delete: Arc::new(SoftDeleteWorkoutTemplate::new(workout_repo.clone())),
            add_exercise: Arc::new(AddExerciseToWorkoutTemplate::new(
                exercise_repo.clone(),
                workout_repo.clone(),
            )),
            remove_exercise: Arc::new(RemoveExerciseFromWorkoutTemplate::new(workout_repo.clone())),
        }
    }
}
