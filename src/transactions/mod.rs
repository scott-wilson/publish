//! The transactions package is responsible for creating new transactions.
//!
//! Transactions are the heart of publish. They are used to publish files,
//! register a publish to the database, etc. A publish is only a collection of
//! transactions and transformations to the data.
//!
//! There are two implemented transactions:
//!
//! - [FilesystemTransaction]: This transaction is used to publish files.
//!   This may include copying, moving, hard/soft linking, changing permissions,
//!   creating folders, etc.
//! - [RootTransaction]: This transaction is used to group other
//!   transactions. It is unlikely that a publish will use this transaction
//!   directly, but could still be useful if transactions need to be done as a
//!   hierarchy of transactions.

mod filesystem_transaction;
mod root_transaction;

pub use filesystem_transaction::{
    FilesystemTransaction, Permission, Permissions, ScopedPermissions,
};
pub use root_transaction::RootTransaction;

/// The transaction interface.
///
/// A transaction is the core of the publish framework. All publishes are a
/// collection of transactions and transformations to the data. Committed
/// transactions may be rolled back if it makes sense to do so. For example, if
/// a file is copied, it can be deleted. However, if a file is deleted, then it
/// cannot un-deleted.
#[async_trait::async_trait]
pub trait Transaction {
    /// Commit the transaction.
    async fn commit(&mut self) -> Result<(), crate::Error>;

    /// Rollback the transaction.
    async fn rollback(&mut self) -> Result<(), crate::Error>;
}
