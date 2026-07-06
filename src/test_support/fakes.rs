use crate::application::errors::CryptoError;
use crate::application::errors::MailError;
use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::CryptoService;
use crate::application::ports::Mailer;
use crate::domain::commands::ExerciseFilterFields;
use crate::domain::commands::ExerciseUpdateFields;
use crate::domain::commands::UserUpdateFields;
use crate::domain::commands::WorkoutPlanFilterFields;
use crate::domain::commands::WorkoutTemplateFilterFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::FinishedWorkoutSession;
use crate::domain::entities::Measurement;
use crate::domain::entities::Paginated;
use crate::domain::entities::PendingChange;
use crate::domain::entities::PendingUser;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlan;
use crate::domain::entities::WorkoutPlanRoutineItem;
use crate::domain::entities::WorkoutPlanSummary;
use crate::domain::entities::WorkoutSession;
use crate::domain::entities::WorkoutSessionExercise;
use crate::domain::entities::WorkoutSessionSet;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::entities::WorkoutTemplateSummary;
use crate::domain::enums::RoutineMode;
use crate::domain::enums::SetType;
use crate::domain::enums::WorkoutSessionStatus;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::ExerciseRepository;
use crate::domain::repositories::MeasurementRepository;
use crate::domain::repositories::PendingChangesRepository;
use crate::domain::repositories::PendingUserRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutSessionRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use crate::test_support::fixtures::clone_session;
use crate::test_support::fixtures::clone_user;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryUserRepository {
    pub users: Mutex<Vec<User>>,
    pub consumed_pending_users: Mutex<Vec<Uuid>>,
    pub consumed_pending_changes: Mutex<Vec<Uuid>>,
}

impl InMemoryUserRepository {
    pub fn with_users(users: Vec<User>) -> Self {
        Self {
            users: Mutex::new(users),
            ..Default::default()
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        self.users.lock().unwrap().push(clone_user(user));
        Ok(())
    }

    async fn create_user_from_pending(
        &self,
        user: &User,
        pending_user_id: Uuid,
    ) -> Result<(), DomainError> {
        self.users.lock().unwrap().push(clone_user(user));
        self.consumed_pending_users
            .lock()
            .unwrap()
            .push(pending_user_id);
        Ok(())
    }

    async fn apply_email_change(
        &self,
        user_id: Uuid,
        email: &str,
        pending_change_id: Uuid,
    ) -> Result<User, DomainError> {
        use crate::domain::value_objects::Email;

        let mut users = self.users.lock().unwrap();
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or_else(|| DomainError::from(RepositoryError::NotFound("user not found".into())))?;
        user.email = Email::new(email.to_string())
            .map_err(|_| RepositoryError::Unexpected("invalid email".into()))?;
        self.consumed_pending_changes
            .lock()
            .unwrap()
            .push(pending_change_id);
        Ok(clone_user(user))
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.id == user_id)
            .map(clone_user)
            .ok_or_else(|| RepositoryError::NotFound("user not found".into()).into())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.email.value() == email)
            .map(clone_user)
            .ok_or_else(|| RepositoryError::NotFound("user not found".into()).into())
    }

    async fn get_user_by_google_sub(&self, _google_sub: &str) -> Result<User, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn link_google_sub(
        &self,
        _user_id: Uuid,
        _google_sub: &str,
        _url_img: Option<String>,
    ) -> Result<User, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        Ok(self.users.lock().unwrap().iter().map(clone_user).collect())
    }

    async fn update_user(
        &self,
        fields: UserUpdateFields,
        user_id: Uuid,
    ) -> Result<User, DomainError> {
        use crate::domain::value_objects::{Email, Name};

        let mut users = self.users.lock().unwrap();
        let user = users
            .iter_mut()
            .find(|u| u.id == user_id)
            .ok_or_else(|| DomainError::from(RepositoryError::NotFound("user not found".into())))?;
        if let Some(name) = fields.name {
            user.name =
                Name::new(name).map_err(|_| RepositoryError::Unexpected("invalid name".into()))?;
        }
        if let Some(email) = fields.email {
            user.email = Email::new(email)
                .map_err(|_| RepositoryError::Unexpected("invalid email".into()))?;
        }
        if let Some(password) = fields.password {
            user.password = password;
        }
        if let Some(url_img) = fields.url_img {
            user.url_img = Some(url_img);
        }
        if let Some(goal) = fields.goal {
            user.goal = Some(goal);
        }
        Ok(clone_user(user))
    }

    async fn soft_delete_user(&self, _user_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn restore_user(&self, _user_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn delete_user(&self, _user_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }
}

fn clone_pending_user(pending_user: &PendingUser) -> PendingUser {
    PendingUser {
        id: pending_user.id,
        name: pending_user.name.clone(),
        email: pending_user.email.clone(),
        password: pending_user.password.clone(),
        code: pending_user.code,
        limit_date: pending_user.limit_date,
    }
}

#[derive(Default)]
pub struct InMemoryPendingUserRepository {
    pub pending_users: Mutex<Vec<PendingUser>>,
}

impl InMemoryPendingUserRepository {
    pub fn with_pending_users(pending_users: Vec<PendingUser>) -> Self {
        Self {
            pending_users: Mutex::new(pending_users),
        }
    }
}

#[async_trait]
impl PendingUserRepository for InMemoryPendingUserRepository {
    async fn create_pending_user(&self, pending_user: PendingUser) -> Result<(), DomainError> {
        self.pending_users.lock().unwrap().push(pending_user);
        Ok(())
    }

    async fn get_valid_pending_user_by_email(
        &self,
        email: &str,
    ) -> Result<PendingUser, DomainError> {
        self.pending_users
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.email == email)
            .map(clone_pending_user)
            .ok_or_else(|| RepositoryError::NotFound("pending user not found".into()).into())
    }

    async fn delete_pending_user(&self, id: Uuid) -> Result<(), DomainError> {
        self.pending_users.lock().unwrap().retain(|p| p.id != id);
        Ok(())
    }

    async fn clear_expired_pending_user(&self) -> Result<(), DomainError> {
        let now = Utc::now();
        self.pending_users
            .lock()
            .unwrap()
            .retain(|p| p.limit_date > now);
        Ok(())
    }
}

pub struct FakeCryptoService;

impl CryptoService for FakeCryptoService {
    fn hash(&self, password: &str) -> Result<String, CryptoError> {
        Ok(format!("hashed:{password}"))
    }

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, CryptoError> {
        Ok(password_hash == format!("hashed:{password}"))
    }
}

#[derive(Default)]
pub struct RecordingMailer {
    pub sent: Mutex<Vec<(String, String, String)>>,
    pub fail: bool,
}

impl RecordingMailer {
    pub fn failing() -> Self {
        Self {
            sent: Mutex::new(Vec::new()),
            fail: true,
        }
    }
}

#[async_trait]
impl Mailer for RecordingMailer {
    async fn send_email(&self, to: &str, subject: &str, code: &str) -> Result<(), MailError> {
        if self.fail {
            return Err(MailError::Send("smtp unavailable".into()));
        }

        self.sent
            .lock()
            .unwrap()
            .push((to.to_string(), subject.to_string(), code.to_string()));

        Ok(())
    }
}

#[derive(Default)]
pub struct InMemoryWorkoutSessionRepository {
    pub sessions: Mutex<Vec<WorkoutSession>>,
    pub finished: Mutex<Vec<FinishedWorkoutSession>>,
}

impl InMemoryWorkoutSessionRepository {
    pub fn with_sessions(sessions: Vec<WorkoutSession>) -> Self {
        Self {
            sessions: Mutex::new(sessions),
            finished: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl WorkoutSessionRepository for InMemoryWorkoutSessionRepository {
    async fn start(&self, session: &WorkoutSession) -> Result<(), DomainError> {
        self.sessions.lock().unwrap().push(clone_session(session));
        Ok(())
    }

    async fn find_by_id(&self, session_id: Uuid) -> Result<WorkoutSession, DomainError> {
        self.sessions
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.id == session_id)
            .map(clone_session)
            .ok_or_else(|| RepositoryError::NotFound("workout session not found".into()).into())
    }

    async fn finish(&self, session: &FinishedWorkoutSession) -> Result<(), DomainError> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(stored) = sessions.iter_mut().find(|s| s.id == session.id) {
            stored.status = session.status;
            stored.finished_at = Some(session.finished_at);
        }

        self.finished.lock().unwrap().push(FinishedWorkoutSession {
            id: session.id,
            status: session.status,
            started_at: session.started_at,
            finished_at: session.finished_at,
        });

        Ok(())
    }

    async fn cancel(&self, _session_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn add_exercise(
        &self,
        _session_id: Uuid,
        _exercise_id: Uuid,
        _client_operation_id: Option<Uuid>,
    ) -> Result<WorkoutSessionExercise, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn add_set(
        &self,
        _session_id: Uuid,
        _exercise_id: Uuid,
        _set_type: SetType,
        _weight: f32,
        _reps: u32,
        _client_operation_id: Option<Uuid>,
        _completed_at: Option<DateTime<Utc>>,
    ) -> Result<WorkoutSessionSet, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn update_set(
        &self,
        _session_id: Uuid,
        _set_id: Uuid,
        _set_type: SetType,
        _weight: f32,
        _reps: u32,
    ) -> Result<WorkoutSessionSet, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn delete_set(&self, _session_id: Uuid, _set_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn reorder_exercises(
        &self,
        _session_id: Uuid,
        _ordered_session_exercise_ids: Vec<Uuid>,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn find_session_exercise_ids(&self, _session_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn remove_exercise(
        &self,
        _session_id: Uuid,
        _session_exercise_id: Uuid,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn has_in_progress(&self, user_id: Uuid) -> Result<bool, DomainError> {
        Ok(self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .any(|s| s.user_id == user_id && s.status == WorkoutSessionStatus::InProgress))
    }
}

pub struct FakeWorkoutPlanRepository {
    pub plan_id: Uuid,
    pub owner_id: Uuid,
    pub saved_plans: Mutex<Vec<String>>,
    pub added_routine_items: Mutex<Vec<Uuid>>,
}

impl FakeWorkoutPlanRepository {
    pub fn new(plan_id: Uuid, owner_id: Uuid) -> Self {
        Self {
            plan_id,
            owner_id,
            saved_plans: Mutex::new(Vec::new()),
            added_routine_items: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl WorkoutPlanRepository for FakeWorkoutPlanRepository {
    async fn save(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        self.saved_plans
            .lock()
            .unwrap()
            .push(workout_plan.name.value().to_string());
        Ok(())
    }

    async fn read_summary(
        &self,
        _filter: WorkoutPlanFilterFields,
    ) -> Result<Vec<WorkoutPlanSummary>, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn find_by_id(&self, workout_plan_id: Uuid) -> Result<WorkoutPlan, DomainError> {
        if workout_plan_id != self.plan_id {
            return Err(RepositoryError::NotFound("workout plan not found".into()).into());
        }

        let mut plan = WorkoutPlan::new(
            self.owner_id,
            "Test Plan".to_string(),
            RoutineMode::Weekly,
            Vec::new(),
        )
        .unwrap();
        plan.id = self.plan_id;

        Ok(plan)
    }

    async fn find_current_user_plan(&self, _user_id: Uuid) -> Result<WorkoutPlan, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn set_current(&self, _user_id: Uuid, _wp_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn update(&self, _workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn delete(&self, _workout_plan_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn soft_delete(&self, _workout_plan_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn add_routine_item(
        &self,
        routine_item: &WorkoutPlanRoutineItem,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        let _ = routine_item;
        self.added_routine_items
            .lock()
            .unwrap()
            .push(workout_plan_id);
        Ok(())
    }

    async fn update_routine_item(
        &self,
        _routine_item: &WorkoutPlanRoutineItem,
        _workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn remove_routine_item(
        &self,
        _workout_plan_id: Uuid,
        _routine_item_id: Uuid,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn remove_workout_template(
        &self,
        _workout_plan_id: Uuid,
        _workout_template_id: Uuid,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }
}

pub struct FakeWorkoutTemplateRepository {
    pub template_id: Uuid,
    pub owner_id: Uuid,
}

#[async_trait]
impl WorkoutTemplateRepository for FakeWorkoutTemplateRepository {
    async fn save(&self, _workout: &WorkoutTemplate) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn read(
        &self,
        _fields: WorkoutTemplateFilterFields,
    ) -> Result<Vec<WorkoutTemplateSummary>, DomainError> {
        unimplemented!("not used in this test")
    }

    async fn find_by_id(&self, workout_id: Uuid) -> Result<WorkoutTemplate, DomainError> {
        if workout_id != self.template_id {
            return Err(RepositoryError::NotFound("workout template not found".into()).into());
        }

        let mut template =
            WorkoutTemplate::new(self.owner_id, "Test Template".to_string(), Vec::new()).unwrap();
        template.id = self.template_id;

        Ok(template)
    }

    async fn update(&self, _workout: &WorkoutTemplate) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn soft_delete(&self, _workout_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn delete(&self, _workout_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn add_exercise(&self, _workout_id: Uuid, _exercise_id: Uuid) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }

    async fn remove_exercise(
        &self,
        _workout_id: Uuid,
        _exercise_id: Uuid,
    ) -> Result<(), DomainError> {
        unimplemented!("not used in this test")
    }
}

pub struct FixedClock(pub DateTime<Utc>);

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.0
    }
}

pub struct FixedCodeGenerator(pub u32);

impl CodeGenerator for FixedCodeGenerator {
    fn verification_code(&self) -> u32 {
        self.0
    }
}

fn clone_measurement(m: &Measurement) -> Measurement {
    m.clone()
}

#[derive(Default)]
pub struct InMemoryMeasurementRepository {
    pub measurements: Mutex<Vec<Measurement>>,
    pub deleted: Mutex<Vec<Uuid>>,
}

impl InMemoryMeasurementRepository {
    pub fn with_measurements(measurements: Vec<Measurement>) -> Self {
        Self {
            measurements: Mutex::new(measurements),
            ..Default::default()
        }
    }
}

#[async_trait]
impl MeasurementRepository for InMemoryMeasurementRepository {
    async fn create_measurement(&self, measurement: Measurement) -> Result<(), DomainError> {
        self.measurements.lock().unwrap().push(measurement);
        Ok(())
    }

    async fn get_measurement_by_id(&self, id: Uuid) -> Result<Measurement, DomainError> {
        self.measurements
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id == id)
            .map(clone_measurement)
            .ok_or_else(|| RepositoryError::NotFound("measurement not found".into()).into())
    }

    async fn get_measurements_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Measurement>, DomainError> {
        Ok(self
            .measurements
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.user_id == user_id)
            .map(clone_measurement)
            .collect())
    }

    async fn soft_delete_measurement(&self, id: Uuid) -> Result<(), DomainError> {
        let mut measurements = self.measurements.lock().unwrap();
        let m = measurements
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| {
                DomainError::from(RepositoryError::NotFound("measurement not found".into()))
            })?;
        m.deleted_at = Some(Utc::now());
        Ok(())
    }

    async fn delete_measurement(&self, id: Uuid) -> Result<(), DomainError> {
        self.measurements.lock().unwrap().retain(|m| m.id != id);
        self.deleted.lock().unwrap().push(id);
        Ok(())
    }
}

#[derive(Default)]
pub struct InMemoryExerciseRepository {
    pub exercises: Mutex<Vec<Exercise>>,
    pub updates: Mutex<Vec<(ExerciseUpdateFields, Option<Uuid>)>>,
    pub soft_deleted: Mutex<Vec<(Uuid, Uuid)>>,
    pub deleted: Mutex<Vec<Uuid>>,
}

impl InMemoryExerciseRepository {
    pub fn with_exercises(exercises: Vec<Exercise>) -> Self {
        Self {
            exercises: Mutex::new(exercises),
            ..Default::default()
        }
    }
}

#[async_trait]
impl ExerciseRepository for InMemoryExerciseRepository {
    async fn create_exercise(&self, exercise: &Exercise) -> Result<(), DomainError> {
        self.exercises.lock().unwrap().push(exercise.clone());
        Ok(())
    }

    async fn get_exercises(
        &self,
        fields: ExerciseFilterFields,
    ) -> Result<Paginated<Exercise>, DomainError> {
        let items: Vec<Exercise> = self
            .exercises
            .lock()
            .unwrap()
            .iter()
            .filter(|e| fields.equipment.is_none() || Some(e.equipment) == fields.equipment)
            .filter(|e| {
                fields.muscle_group.is_none() || Some(e.muscle_group) == fields.muscle_group
            })
            .filter(|e| {
                fields.exercise_type.is_none() || Some(e.exercise_type) == fields.exercise_type
            })
            .cloned()
            .collect();
        let total = items.len() as i64;
        let pagination = fields.pagination.expect("pagination is always set");
        Ok(Paginated::new(
            items,
            total,
            pagination.per_page,
            pagination.page,
        ))
    }

    async fn read_by_id(&self, exercise_id: Uuid) -> Result<Exercise, DomainError> {
        self.exercises
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.id == exercise_id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound("exercise not found".into()).into())
    }

    async fn update_exercise(
        &self,
        fields: ExerciseUpdateFields,
        user_id: Option<Uuid>,
    ) -> Result<(), DomainError> {
        self.updates.lock().unwrap().push((fields, user_id));
        Ok(())
    }

    async fn soft_delete_exercise(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        self.soft_deleted.lock().unwrap().push((id, user_id));
        Ok(())
    }

    async fn delete_exercise(&self, id: Uuid) -> Result<(), DomainError> {
        self.deleted.lock().unwrap().push(id);
        Ok(())
    }
}

fn clone_pending_change(pc: &PendingChange) -> PendingChange {
    PendingChange {
        id: pc.id,
        user_id: pc.user_id,
        code: pc.code,
        limit_date: pc.limit_date,
    }
}

#[derive(Default)]
pub struct InMemoryPendingChangeRepository {
    pub pending_changes: Mutex<Vec<PendingChange>>,
}

impl InMemoryPendingChangeRepository {
    pub fn with_pending_changes(pending_changes: Vec<PendingChange>) -> Self {
        Self {
            pending_changes: Mutex::new(pending_changes),
        }
    }
}

#[async_trait]
impl PendingChangesRepository for InMemoryPendingChangeRepository {
    async fn create_pending_change(
        &self,
        pending_change: PendingChange,
    ) -> Result<(), DomainError> {
        self.pending_changes.lock().unwrap().push(pending_change);
        Ok(())
    }

    async fn get_valid_pending_change_by_user_id(
        &self,
        id: Uuid,
    ) -> Result<PendingChange, DomainError> {
        self.pending_changes
            .lock()
            .unwrap()
            .iter()
            .find(|pc| pc.user_id == id)
            .map(clone_pending_change)
            .ok_or_else(|| RepositoryError::NotFound("pending change not found".into()).into())
    }

    async fn delete_pending_change(&self, id: Uuid) -> Result<(), DomainError> {
        self.pending_changes
            .lock()
            .unwrap()
            .retain(|pc| pc.id != id);
        Ok(())
    }

    async fn clear_expired_pending_change(&self) -> Result<(), DomainError> {
        let now = Utc::now();
        self.pending_changes
            .lock()
            .unwrap()
            .retain(|pc| pc.limit_date > now);
        Ok(())
    }
}
