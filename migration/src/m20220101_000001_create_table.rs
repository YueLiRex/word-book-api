use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let _ = manager
      .create_table(
        Table::create()
          .table(Words::Table)
          .if_not_exists()
          .col(pk_auto(Words::Id))
          .col(string(Words::Word))
          .col(boolean(Words::IsSelected))
          .col(timestamp(Words::CreatedAt))
          .col(timestamp(Words::UpdatedAt))
          .to_owned(),
      )
      .await;

    let _ = manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(pk_uuid(Users::Id))
          .col(string(Users::Email))
          .col(string(Users::Password))
          .col(timestamp(Users::CreatedAt))
          .col(timestamp(Users::UpdatedAt))
          .to_owned(),
      )
      .await;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let _ = manager
      .drop_table(Table::drop().table(Words::Table).to_owned())
      .await;

    let _ = manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await;

    Ok(())
  }
}

#[derive(DeriveIden)]
enum Words {
  Table,
  Id,
  Word,
  IsSelected,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
  Table,
  Id,
  Email,
  Password,
  CreatedAt,
  UpdatedAt,
}
