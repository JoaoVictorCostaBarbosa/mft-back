use crate::domain::entities::User;
use crate::domain::entities::WorkoutSession;
use uuid::Uuid;

pub fn user() -> User {
    User::new(
        "Test User".to_string(),
        "user@test.com".to_string(),
        "hashed:Password1".to_string(),
    )
    .unwrap()
}

pub fn admin() -> User {
    let mut user = user();
    user.role = crate::domain::enums::Role::Admin;
    user
}

pub fn user_with_email(email: &str) -> User {
    User::new(
        "Test User".to_string(),
        email.to_string(),
        "hashed:Password1".to_string(),
    )
    .unwrap()
}

pub fn clone_user(user: &User) -> User {
    User {
        id: user.id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: user.password.clone(),
        google_sub: user.google_sub.clone(),
        role: user.role,
        url_img: user.url_img.clone(),
        goal: user.goal,
        created_at: user.created_at,
        updated_at: user.updated_at,
        deleted_at: user.deleted_at,
    }
}

pub fn in_progress_session(user_id: Uuid) -> WorkoutSession {
    WorkoutSession::start(user_id, Some(Uuid::new_v4()), Some(Uuid::new_v4()))
}

pub fn clone_session(session: &WorkoutSession) -> WorkoutSession {
    WorkoutSession {
        id: session.id,
        user_id: session.user_id,
        workout_plan_id: session.workout_plan_id,
        workout_template_id: session.workout_template_id,
        started_at: session.started_at,
        finished_at: session.finished_at,
        status: session.status,
    }
}
