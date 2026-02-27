use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create web3_wallet table
        manager
            .create_table(
                Table::create()
                    .table(Web3Wallet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Web3Wallet::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .comment("主键ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::UserId)
                            .string()
                            .nullable()
                            .comment("关联用户ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::WalletAddress)
                            .string()
                            .not_null()
                            .comment("钱包地址"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::WalletType)
                            .string()
                            .not_null()
                            .default("metamask")
                            .comment("钱包类型"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::ChainId)
                            .integer()
                            .not_null()
                            .default(1)
                            .comment("链ID"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::Signature)
                            .string()
                            .nullable()
                            .comment("签名"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::Message)
                            .string()
                            .nullable()
                            .comment("签名消息"),
                    )
                    .col(
                        ColumnDef::new(Web3Wallet::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(ColumnDef::new(Web3Wallet::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create index for wallet address
        manager
            .create_index(
                Index::create()
                    .table(Web3Wallet::Table)
                    .name("idx_web3_wallet_address")
                    .col(Web3Wallet::WalletAddress)
                    .to_owned(),
            )
            .await?;

        // Create index for user_id
        manager
            .create_index(
                Index::create()
                    .table(Web3Wallet::Table)
                    .name("idx_web3_wallet_user_id")
                    .col(Web3Wallet::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Web3Wallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Web3Wallet {
    Table,
    Id,
    UserId,
    WalletAddress,
    WalletType,
    ChainId,
    Signature,
    Message,
    CreatedAt,
    UpdatedAt,
}
