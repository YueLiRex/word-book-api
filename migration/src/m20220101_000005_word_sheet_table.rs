use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let _ = manager
      .create_table(
        Table::create()
          .table(Sheets::Table)
          .if_not_exists()
          .col(pk_auto(Sheets::Id))
          .col(string(Sheets::Name).not_null())
          .col(decimal(Sheets::Score))
          .col(boolean(Sheets::IsFinished).default(false))
          .col(timestamp(Sheets::CreatedAt))
          .col(timestamp(Sheets::UpdatedAt))
          .col(uuid(Sheets::UserId).not_null())
          .to_owned(),
      )
      .await?;

    let _ = manager
      .create_foreign_key(
        ForeignKey::create()
          .name("fk_sheets_user_id")
          .from_tbl(Sheets::Table)
          .from_col(Sheets::UserId)
          .to_tbl(Users::Table)
          .to_col(Users::Id)
          .on_delete(ForeignKeyAction::Cascade)
          .to_owned(),
      )
      .await?;

    let _ = manager
      .create_table(
        Table::create()
          .table(SheetsWords::Table)
          .if_not_exists()
          .col(pk_auto(SheetsWords::Id))
          .col(integer(SheetsWords::SheetId).not_null())
          .col(integer(SheetsWords::WordId).not_null())
          .foreign_key(
            ForeignKey::create()
              .name("fk_sheets_words_sheet_id")
              .from_tbl(SheetsWords::Table)
              .from_col(SheetsWords::SheetId)
              .to_tbl(Sheets::Table)
              .to_col(Sheets::Id)
              .on_delete(ForeignKeyAction::Cascade)
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk_sheets_words_word_id")
              .from_tbl(SheetsWords::Table)
              .from_col(SheetsWords::WordId)
              .to_tbl(Words::Table)
              .to_col(Words::Id)
              .on_delete(ForeignKeyAction::Cascade)
          )
          .to_owned(),
      ).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let _ = manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("fk_sheets_user_id")
          .table(Sheets::Table)
          .to_owned(),
      )
      .await?;

    let _ = manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("fk_sheets_words_sheet_id")
          .table(SheetsWords::Table)
          .to_owned(),
      )
      .await?; 

    let _ = manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("fk_sheets_words_word_id")
          .table(SheetsWords::Table)
          .to_owned(),
      )
      .await?;

    let _ = manager
      .drop_table(Table::drop().table(SheetsWords::Table).to_owned())
      .await?;

    let _ = manager
      .drop_table(Table::drop().table(Sheets::Table).to_owned())
      .await?;

    if manager.has_column("sheets", "user_id").await? {
      manager
        .alter_table(
          Table::alter()
            .table(Sheets::Table)
            .drop_column(Sheets::UserId)
            .to_owned(),
        )
        .await?;
    }

    Ok(())
  }
}

#[derive(DeriveIden)]
enum Sheets {
  Table,
  Id,
  Name,
  Score,
  IsFinished,
  CreatedAt,
  UpdatedAt,
  UserId,
}

#[derive(DeriveIden)]
enum Users {
  Table,
  Id,
}

#[derive(DeriveIden)]
enum Words {
  Table,
  Id,
}

#[derive(DeriveIden)]
enum SheetsWords {
  Table,
  Id,
  SheetId,
  WordId,
}
