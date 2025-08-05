use sea_orm::ConnectionTrait;

pub struct UsersRepo<'a, C: ConnectionTrait> {
    pub conn: &'a C,
}

impl<'a, C: ConnectionTrait> UsersRepo<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }
}
