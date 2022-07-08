use crate::errors::ApiError;
use crate::model::{HttpUser, PagedResult};

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, wanted_username: &str) -> Result<Option<users::Model>, ApiError> {
        Ok(User::find()
            .filter(users::Column::Username.eq(wanted_username))
            .one(&self.db)
            .await?)
    }

    pub async fn list_users(
        &self,
        page: usize,
        page_size: usize,
    ) -> Result<PagedResult<HttpUser>, ApiError> {
        let user_paginator = User::find()
            .into_model::<HttpUser>()
            .paginate(&self.db, page_size);

        let total_pages = user_paginator.num_pages().await?;
        let total_items = user_paginator.num_items().await?;
        let content = user_paginator.fetch_page(page - 1).await?;
        let elements_number = content.len();

        Ok(PagedResult {
            content,
            page,
            page_size,
            total_pages,
            elements_number,
            total_items,
        })
    }

    pub async fn create_user(
        &self,
        login: &str,
        pwd: &str,
        user_role: UserRole,
    ) -> Result<users::Model, ApiError> {
        let new_user = users::ActiveModel {
            id: NotSet,
            username: Set(String::from(login)),
            password: Set(encode_password(pwd)),
            role: Set(user_role),
        };

        Ok(new_user.insert(&self.db).await?)
    }
}

fn encode_password(pwd: &str) -> String {
    let salt = std::env::var("PASSWORD_SALT").unwrap_or_else(|_| String::from("lepetitcerebos"));
    let config = argon2::Config::default();

    argon2::hash_encoded(pwd.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn match_password(user: &users::Model, candidate: &str) -> bool {
    argon2::verify_encoded(&user.password, candidate.as_bytes()).unwrap()
}
