use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

use crate::{entity::users, provider::types::idp::OAuthProvider};

pub struct UsersRepo<'a, C: ConnectionTrait> {
    pub conn: &'a C,
}

impl<'a, C: ConnectionTrait> UsersRepo<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }

    pub async fn get_or_create_user_if_not_exist(
        &self,
        idp: OAuthProvider,
        idp_uid: String,
    ) -> Result<users::Model, DbErr> {
        let existing_user = self
            .get_user_by_idp_and_idp_uid(idp.clone(), idp_uid.clone())
            .await?;

        if let Some(user) = existing_user {
            return Ok(user);
        }

        let now = chrono::Utc::now().into();
        let new_user = users::ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(None),
            email: Set(None),
            is_active: Set(true),
            idp: Set(idp.as_str().to_string()),
            idp_uid: Set(idp_uid),
            created_at: Set(now),
            updated_at: Set(now),
        };
        new_user.insert(self.conn).await
    }

    pub async fn get_user_by_idp_and_idp_uid(
        &self,
        idp: OAuthProvider,
        idp_uid: String,
    ) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(
                users::Column::Idp
                    .eq(idp.as_str())
                    .and(users::Column::IdpUid.eq(idp_uid)),
            )
            .one(self.conn)
            .await
    }
}
