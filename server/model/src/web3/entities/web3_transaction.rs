//! `SeaORM` Entity, for Web3 Transaction

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "web3_transaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub contract_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub method_name: String,
    #[sea_orm(column_type = "Json", nullable)]
    pub params: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub tx_hash: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub status: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub from_address: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub error_message: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
