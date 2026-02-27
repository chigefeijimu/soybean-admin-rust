use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create web3_transaction table
        manager
            .create_table(
                Table::create()
                    .table(Web3Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Web3Transaction::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .comment("主键ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::UserId)
                            .string()
                            .nullable()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::ContractId)
                            .string()
                            .nullable()
                            .comment("合约ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::MethodName)
                            .string()
                            .not_null()
                            .comment("调用的方法名"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::Params)
                            .json()
                            .nullable()
                            .comment("参数"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::TxHash)
                            .string()
                            .nullable()
                            .comment("交易哈希"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::Status)
                            .string()
                            .not_null()
                            .default("pending")
                            .comment("状态: pending, confirmed, failed"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::FromAddress)
                            .string()
                            .nullable()
                            .comment("调用者地址"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::ErrorMessage)
                            .text()
                            .nullable()
                            .comment("错误信息"),
                    )
                    .col(
                        ColumnDef::new(Web3Transaction::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(ColumnDef::new(Web3Transaction::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create index for user_id
        manager
            .create_index(
                Index::create()
                    .table(Web3Transaction::Table)
                    .name("idx_web3_transaction_user_id")
                    .col(Web3Transaction::UserId)
                    .to_owned(),
            )
            .await?;

        // Create index for contract_id
        manager
            .create_index(
                Index::create()
                    .table(Web3Transaction::Table)
                    .name("idx_web3_transaction_contract_id")
                    .col(Web3Transaction::ContractId)
                    .to_owned(),
            )
            .await?;

        // Create index for tx_hash
        manager
            .create_index(
                Index::create()
                    .table(Web3Transaction::Table)
                    .name("idx_web3_transaction_tx_hash")
                    .col(Web3Transaction::TxHash)
                    .to_owned(),
            )
            .await?;

        // Create index for status
        manager
            .create_index(
                Index::create()
                    .table(Web3Transaction::Table)
                    .name("idx_web3_transaction_status")
                    .col(Web3Transaction::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Web3Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Web3Transaction {
    Table,
    Id,
    UserId,
    ContractId,
    MethodName,
    Params,
    TxHash,
    Status,
    FromAddress,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}
