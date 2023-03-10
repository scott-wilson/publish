#[async_trait::async_trait]
pub trait Publish {
    type Context: Send + Sync;

    async fn pre_publish(
        &self,
        _transaction: &mut crate::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, crate::Error> {
        Ok(context)
    }

    async fn publish(
        &self,
        transaction: &mut crate::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, crate::Error>;

    async fn post_publish(
        &self,
        _transaction: &mut crate::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, crate::Error> {
        Ok(context)
    }
}
