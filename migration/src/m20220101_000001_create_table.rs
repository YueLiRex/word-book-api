use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let _ = manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(pk_uuid(Article::Id))
                    .col(string(Article::Title))
                    .col(boolean(Article::IsDraft))
                    .col(string(Article::Content))
                    .col(timestamp(Article::CreatedAt))
                    .col(timestamp(Article::UpdatedAt))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string(User::Email))
                    .col(string(User::Password))
                    .col(timestamp(User::CreatedAt))
                    .col(timestamp(User::UpdatedAt))
                    .to_owned(),
            )
        .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let _ = manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await;

        let _ = manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    Title,
    IsDraft,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
