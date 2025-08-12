use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    if !manager.has_column("words", "is_finished").await? {
      manager
        .alter_table(
          Table::alter()
            .table(Words::Table)
            .add_column(ColumnDef::new(Words::IsFinished).boolean().default(false))
            .to_owned(),
        )
        .await?;
    }

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

    if manager.has_column("words", "is_finished").await? {
      manager
        .alter_table(
          Table::alter()
            .table(Words::Table)
            .drop_column(Words::IsFinished)
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
  IsFinished,
}
