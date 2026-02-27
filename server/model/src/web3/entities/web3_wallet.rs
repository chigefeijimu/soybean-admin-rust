//! `SeaORM` Entity, for Web3 Wallet

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "web3_wallet")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub wallet_address: String,
    #[sea_orm(column_type = "Text")]
    pub wallet_type: String,
    #[sea_orm(column_type = "Int")]
    pub chain_id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub signature: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub message: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
