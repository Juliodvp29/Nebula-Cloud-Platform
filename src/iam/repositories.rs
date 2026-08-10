use crate::error::AppError;
use crate::iam::domain::action::Action;
use crate::iam::domain::membership::{Membership, MembershipStatus};
use crate::iam::domain::organization::{Organization, OrganizationStatus};
use crate::iam::domain::policy::Policy;
use crate::iam::domain::role::Role;
use crate::iam::domain::user::{User, UserStatus};
use crate::platform::{ActionId, OrganizationId, PolicyId, RoleId, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn create(&self, org: &Organization) -> Result<(), AppError>;
    async fn find_by_id(&self, id: OrganizationId) -> Result<Option<Organization>, AppError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Organization>, AppError>;
    async fn update(&self, org: &Organization) -> Result<(), AppError>;
    async fn update_status(
        &self,
        id: OrganizationId,
        status: OrganizationStatus,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn update(&self, user: &User) -> Result<(), AppError>;
    async fn update_status(&self, id: UserId, status: UserStatus) -> Result<(), AppError>;
    async fn update_last_login(&self, id: UserId) -> Result<(), AppError>;
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: &Role) -> Result<(), AppError>;
    async fn find_by_id(&self, id: RoleId) -> Result<Option<Role>, AppError>;
    async fn find_by_name(
        &self,
        organization_id: OrganizationId,
        name: &str,
    ) -> Result<Option<Role>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Role>, AppError>;
    async fn update(&self, role: &Role) -> Result<(), AppError>;
    async fn delete(&self, id: RoleId) -> Result<(), AppError>;
}

#[async_trait]
pub trait PolicyRepository: Send + Sync {
    async fn create(&self, policy: &Policy) -> Result<(), AppError>;
    async fn find_by_id(&self, id: PolicyId) -> Result<Option<Policy>, AppError>;
    async fn find_by_name(
        &self,
        organization_id: OrganizationId,
        name: &str,
    ) -> Result<Option<Policy>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Policy>, AppError>;
    async fn update(&self, policy: &Policy) -> Result<(), AppError>;
    async fn delete(&self, id: PolicyId) -> Result<(), AppError>;
}

#[async_trait]
pub trait ActionRepository: Send + Sync {
    async fn create(&self, action: &Action) -> Result<(), AppError>;
    async fn find_by_id(&self, id: ActionId) -> Result<Option<Action>, AppError>;
    async fn find_by_service(&self, service: &str) -> Result<Vec<Action>, AppError>;
    async fn find_by_full_name(&self, full_name: &str) -> Result<Option<Action>, AppError>;
    async fn list_all(&self) -> Result<Vec<Action>, AppError>;
}

#[async_trait]
pub trait MembershipRepository: Send + Sync {
    async fn add(&self, membership: &Membership) -> Result<(), AppError>;
    async fn remove(
        &self,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<(), AppError>;
    async fn find(
        &self,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Result<Option<Membership>, AppError>;
    async fn find_by_organization(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<Membership>, AppError>;
    async fn find_by_user(&self, user_id: UserId) -> Result<Vec<Membership>, AppError>;
    async fn update_status(
        &self,
        organization_id: OrganizationId,
        user_id: UserId,
        status: MembershipStatus,
    ) -> Result<(), AppError>;
}

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn assign(&self, user_id: UserId, role_id: RoleId) -> Result<(), AppError>;
    async fn revoke(&self, user_id: UserId, role_id: RoleId) -> Result<(), AppError>;
    async fn find_by_user(&self, user_id: UserId) -> Result<Vec<RoleId>, AppError>;
    async fn find_by_role(&self, role_id: RoleId) -> Result<Vec<UserId>, AppError>;
}

#[async_trait]
pub trait RolePolicyRepository: Send + Sync {
    async fn attach(&self, role_id: RoleId, policy_id: PolicyId) -> Result<(), AppError>;
    async fn detach(&self, role_id: RoleId, policy_id: PolicyId) -> Result<(), AppError>;
    async fn find_by_role(&self, role_id: RoleId) -> Result<Vec<PolicyId>, AppError>;
    async fn find_by_policy(&self, policy_id: PolicyId) -> Result<Vec<RoleId>, AppError>;
}
