pub mod admin;
pub mod web3;

// 只导出 prelude，避免命名冲突
pub use web3::entities::prelude::*;
