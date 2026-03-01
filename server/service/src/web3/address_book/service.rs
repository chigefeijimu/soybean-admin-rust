// =========================================
// Address Book Service
// =========================================

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, Order, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use server_model::web3::entities::address_book::{Entity, Model, ActiveModel, Column};

use crate::web3::address_book::error::{ServiceError, ServiceResult};

// ============ Input Types ============

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAddressInput {
    pub label: String,
    pub address: String,
    pub chain_id: i32,
    pub address_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAddressInput {
    pub id: String,
    pub label: Option<String>,
    pub address: Option<String>,
    pub chain_id: Option<i32>,
    pub address_type: Option<String>,
    pub description: Option<String>,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressBookEntry {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub address: String,
    pub chain_id: i32,
    pub address_type: Option<String>,
    pub description: Option<String>,
    pub is_favorite: bool,
    pub created_at: String,
    pub updated_at: String,
    pub ens_name: Option<String>,
    pub balance: Option<String>,
}

impl From<Model> for AddressBookEntry {
    fn from(model: Model) -> Self {
        AddressBookEntry {
            id: model.id,
            user_id: model.user_id,
            label: model.label,
            address: model.address,
            chain_id: model.chain_id,
            address_type: model.address_type,
            description: model.description,
            is_favorite: model.is_favorite,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            ens_name: None,
            balance: None,
        }
    }
}

// ============ Address Book Service ============

pub struct AddressBookService {
    db: Arc<DatabaseConnection>,
}

impl AddressBookService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create_address(&self, user_id: &str, input: CreateAddressInput) -> ServiceResult<AddressBookEntry> {
        let now = Utc::now().naive_utc();
        
        let entry = ActiveModel {
            id: Set(ulid::Ulid::new().to_string()),
            user_id: Set(user_id.to_string()),
            label: Set(input.label),
            address: Set(input.address.to_lowercase()),
            chain_id: Set(input.chain_id),
            address_type: Set(input.address_type),
            description: Set(input.description),
            is_favorite: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let result = entry.insert(&*self.db).await.map_err(|e| ServiceError::new(&e.to_string()))?;
        Ok(result.into())
    }

    pub async fn update_address(&self, input: UpdateAddressInput) -> ServiceResult<AddressBookEntry> {
        let existing = Entity::find_by_id(&input.id)
            .one(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Address not found"))?;

        let mut active_model: ActiveModel = existing.into();
        
        if let Some(label) = input.label {
            active_model.label = Set(label);
        }
        if let Some(address) = input.address {
            active_model.address = Set(address.to_lowercase());
        }
        if let Some(chain_id) = input.chain_id {
            active_model.chain_id = Set(chain_id);
        }
        if let Some(address_type) = input.address_type {
            active_model.address_type = Set(Some(address_type));
        }
        if let Some(description) = input.description {
            active_model.description = Set(Some(description));
        }
        if let Some(is_favorite) = input.is_favorite {
            active_model.is_favorite = Set(is_favorite);
        }
        
        active_model.updated_at = Set(Utc::now().naive_utc());
        
        let result = active_model.update(&*self.db).await.map_err(|e| ServiceError::new(&e.to_string()))?;
        Ok(result.into())
    }

    pub async fn delete_address(&self, id: &str) -> ServiceResult<bool> {
        let result = Entity::delete_by_id(id)
            .exec(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;
        
        Ok(result.rows_affected > 0)
    }

    pub async fn get_address_by_id(&self, id: &str) -> ServiceResult<AddressBookEntry> {
        let entry = Entity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Address not found"))?;

        Ok(entry.into())
    }

    pub async fn list_addresses(&self, user_id: &str) -> ServiceResult<Vec<AddressBookEntry>> {
        let entries = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .order_by(Column::IsFavorite, Order::Desc)
            .order_by(Column::Label, Order::Asc)
            .all(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(entries.into_iter().map(|e| e.into()).collect())
    }

    pub async fn search_addresses(&self, user_id: &str, query: &str) -> ServiceResult<Vec<AddressBookEntry>> {
        let search_pattern = format!("%{}%", query.to_lowercase());
        
        let entries = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::Label.like(&search_pattern))
            .order_by(Column::IsFavorite, Order::Desc)
            .all(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(entries.into_iter().map(|e| e.into()).collect())
    }

    pub async fn toggle_favorite(&self, id: &str) -> ServiceResult<AddressBookEntry> {
        let existing = Entity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Address not found"))?;

        let is_fav = existing.is_favorite;
        let mut active_model: ActiveModel = existing.into();
        active_model.is_favorite = Set(!is_fav);
        active_model.updated_at = Set(Utc::now().naive_utc());
        
        let result = active_model.update(&*self.db).await.map_err(|e| ServiceError::new(&e.to_string()))?;
        Ok(result.into())
    }
}

// ============ Trait ============

#[async_trait]
pub trait TAddressBookService: Send + Sync {
    fn new(db: Arc<DatabaseConnection>) -> Self;
    async fn create_address(&self, user_id: &str, input: CreateAddressInput) -> ServiceResult<AddressBookEntry>;
    async fn update_address(&self, input: UpdateAddressInput) -> ServiceResult<AddressBookEntry>;
    async fn delete_address(&self, id: &str) -> ServiceResult<bool>;
    async fn get_address_by_id(&self, id: &str) -> ServiceResult<AddressBookEntry>;
    async fn list_addresses(&self, user_id: &str) -> ServiceResult<Vec<AddressBookEntry>>;
    async fn search_addresses(&self, user_id: &str, query: &str) -> ServiceResult<Vec<AddressBookEntry>>;
    async fn toggle_favorite(&self, id: &str) -> ServiceResult<AddressBookEntry>;
}

#[async_trait]
impl TAddressBookService for AddressBookService {
    fn new(db: Arc<DatabaseConnection>) -> Self {
        Self::new(db)
    }

    async fn create_address(&self, user_id: &str, input: CreateAddressInput) -> ServiceResult<AddressBookEntry> {
        self.create_address(user_id, input).await
    }

    async fn update_address(&self, input: UpdateAddressInput) -> ServiceResult<AddressBookEntry> {
        self.update_address(input).await
    }

    async fn delete_address(&self, id: &str) -> ServiceResult<bool> {
        self.delete_address(id).await
    }

    async fn get_address_by_id(&self, id: &str) -> ServiceResult<AddressBookEntry> {
        self.get_address_by_id(id).await
    }

    async fn list_addresses(&self, user_id: &str) -> ServiceResult<Vec<AddressBookEntry>> {
        self.list_addresses(user_id).await
    }

    async fn search_addresses(&self, user_id: &str, query: &str) -> ServiceResult<Vec<AddressBookEntry>> {
        self.search_addresses(user_id, query).await
    }

    async fn toggle_favorite(&self, id: &str) -> ServiceResult<AddressBookEntry> {
        self.toggle_favorite(id).await
    }
}
