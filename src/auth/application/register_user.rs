use crate::auth::domain::password::hash_password;
use crate::auth::domain::repositories::UserRepository;
use crate::auth::domain::user::{User, UserStatus};
use crate::error::AppError;
use crate::platform::{OrganizationId, UserId};
use chrono::Utc;
use uuid::Uuid;

pub struct RegisterUserInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub organization_id: OrganizationId,
}

pub struct RegisterUserOutput {
    pub user: User,
}

pub async fn register_user<R: UserRepository>(
    repo: &R,
    input: RegisterUserInput,
) -> Result<RegisterUserOutput, AppError> {
    if repo.find_by_email(&input.email).await?.is_some() {
        return Err(AppError::Validation("email already registered".to_string()));
    }
    if repo.find_by_username(&input.username).await?.is_some() {
        return Err(AppError::Validation("username already taken".to_string()));
    }

    let password_hash = hash_password(&input.password)?;
    let now = Utc::now();
    let user = User {
        id: UserId(Uuid::new_v4()),
        email: input.email,
        username: input.username,
        password_hash,
        status: UserStatus::Active,
        last_login_at: None,
        created_at: now,
        updated_at: now,
    };

    repo.create(&user).await?;
    Ok(RegisterUserOutput { user })
}
