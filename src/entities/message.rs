//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

#[cfg(feature = "ssr")]
pub mod server {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "message")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub message_id: i32,
        pub message_body: String,
        pub message_image: Option<String>,
        pub message_created_at: DateTimeUtc,
        pub message_conversation_id: i32,
        pub message_sender_id: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "crate::entities::conversation::server::Entity",
            from = "Column::MessageConversationId",
            to = "crate::entities::conversation::server::Column::Id",
            on_update = "Restrict",
            on_delete = "Cascade"
        )]
        Conversation,
        #[sea_orm(
            belongs_to = "crate::entities::users::server::Entity",
            from = "Column::MessageSenderId",
            to = "crate::entities::users::server::Column::Id",
            on_update = "Restrict",
            on_delete = "Cascade"
        )]
        Users,
    }

    impl Related<crate::entities::conversation::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Conversation.def()
        }
    }

    impl Related<crate::entities::users::server::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Users.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
