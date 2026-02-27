#![allow(dead_code)]

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create web3_contract table
        manager
            .create_table(
                Table::create()
                    .table(Web3Contract::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Web3Contract::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .comment("主键ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::Name)
                            .string()
                            .not_null()
                            .comment("合约名称"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::ContractAddress)
                            .string()
                            .not_null()
                            .comment("合约地址"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::ChainId)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("链ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::Abi)
                            .json()
                            .null()
                            .comment("合约ABI"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::Description)
                            .string()
                            .null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::CreatedBy)
                            .string()
                            .null()
                            .comment("创建者"),
                    )
                    .col(
                        ColumnDef::new(Web3Contract::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(ColumnDef::new(Web3Contract::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create index for contract address
        manager
            .create_index(
                Index::create()
                    .table(Web3Contract::Table)
                    .name("idx_web3_contract_address")
                    .col(Web3Contract::ContractAddress)
                    .to_owned(),
            )
            .await?;

        // Create index for chain_id
        manager
            .create_index(
                Index::create()
                    .table(Web3Contract::Table)
                    .name("idx_web3_contract_chain_id")
                    .col(Web3Contract::ChainId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Web3Contract::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Web3Contract {
    Table,
    Id,
    Name,
    ContractAddress,
    ChainId,
    Abi,
    Description,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
