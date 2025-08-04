use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    if !manager.has_column("words", "user_id").await? {
      manager
        .alter_table(
          Table::alter()
            .table(Words::Table)
            .add_column(ColumnDef::new(Words::UserId).uuid().not_null())
            .to_owned(),
        )
        .await?;
    }

    manager
      .create_foreign_key(
        sea_query::ForeignKey::create()
          .name("fk_words_user_id")
          .from_tbl(Words::Table)
          .from_col(Words::UserId)
          .to_tbl(Users::Table)
          .to_col(Users::Id)
          .on_delete(sea_query::ForeignKeyAction::Cascade)
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("fk_words_user_id")
          .table(Words::Table)
          .to_owned(),
      )
      .await?;

    if manager.has_column("words", "user_id").await? {
      manager
        .alter_table(
          Table::alter()
            .table(Words::Table)
            .drop_column(Words::UserId)
            .to_owned(),
        )
        .await?;
    }

    Ok(())
  }
}

#[derive(DeriveIden)]
enum Words {
  Table,
  UserId,
}

#[derive(DeriveIden)]
enum Users {
  Table,
  Id,
}
