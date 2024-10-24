use ::entity::{user, user::Entity as User};
use sea_orm::{prelude::Uuid, DbConn, DbErr, EntityTrait};

pub struct Query;

impl Query {
    pub async fn get_user_by_id(db: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).into_model().one(db).await
    }
}
