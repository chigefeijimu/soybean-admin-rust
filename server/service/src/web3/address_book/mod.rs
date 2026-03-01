pub mod error;
pub mod service;

pub use error::{ServiceError, ServiceResult};
pub use service::{
    AddressBookService, CreateAddressInput, UpdateAddressInput, AddressBookEntry,
    TAddressBookService,
};
