use ::entity::{user, user::Entity as User};
use sea_orm::*;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

use crate::forms::UserForm;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, form: UserForm) -> Result<user::Model, sea_orm::DbErr> {
        user::ActiveModel {
            id: Set(Uuid::now_v7()),
            email: Set(form.email.clone()),
            password: Set(hash(form.password.clone(), DEFAULT_COST).unwrap().into()),
            name: Set(form.name.clone()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        User::delete_by_id(id).exec(db).await
    }
}
