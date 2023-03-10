mod filesystem_transaction;
mod root_transaction;

pub use filesystem_transaction::{
    FilesystemTransaction, Permission, Permissions, ScopedPermissions,
};
pub use root_transaction::RootTransaction;

#[async_trait::async_trait]
pub trait Transaction {
    async fn commit(&mut self) -> Result<(), crate::Error>;
    async fn rollback(&mut self) -> Result<(), crate::Error>;
}
