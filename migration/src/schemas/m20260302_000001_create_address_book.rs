use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
#[allow(dead_code)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AddressBook::Table)
                    .col(
                        ColumnDef::new(AddressBook::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::Label)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::Address)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::ChainId)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(AddressBook::AddressType)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::Description)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::IsFavorite)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AddressBook::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AddressBook::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for faster queries
        manager
            .create_index(
                Index::create()
                    .table(AddressBook::Table)
                    .col(AddressBook::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(AddressBook::Table)
                    .col(AddressBook::Address)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AddressBook::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
#[allow(dead_code)]
enum AddressBook {
    Table,
    Id,
    UserId,
    Label,
    Address,
    ChainId,
    AddressType,
    Description,
    IsFavorite,
    CreatedAt,
    UpdatedAt,
}
