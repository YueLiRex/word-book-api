use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let _ = manager
      .create_table(
        Table::create()
          .table(Profiles::Table)
          .if_not_exists()
          .col(pk_auto(Profiles::Id))
          .col(string(Profiles::UserId).uuid().not_null())
          .col(boolean(Profiles::Username))
          .col(string(Profiles::Avatar).default("default_avatar.png"))
          .col(string(Profiles::Nickname))
          .col(string(Profiles::Roles).default("[]"))
          .col(string(Profiles::Permissions).default("[]"))
          .col(timestamp(Profiles::CreatedAt))
          .col(timestamp(Profiles::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk_profile_user_id")
              .from_tbl(Profiles::Table)
              .from_col(Profiles::UserId)
              .to_tbl(Users::Table)
              .to_col(Users::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("fk_profile_user_id")
          .table(Profiles::Table)
          .to_owned(),
      )
      .await?;

    manager
      .drop_table(Table::drop().table(Profiles::Table).to_owned())
      .await?;

    Ok(())
  }
}

#[derive(DeriveIden)]
enum Profiles {
  Table,
  Id,
  UserId,
  Username,
  Avatar,
  Nickname,
  Roles,
  Permissions,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
  Table,
  Id,
}
