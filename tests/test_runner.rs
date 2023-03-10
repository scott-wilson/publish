struct TestPublish;

#[async_trait::async_trait]
impl publish::Publish for TestPublish {
    type Context = u8;

    async fn pre_publish(
        &self,
        _transaction: &mut publish::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, publish::Error> {
        Ok(context + 1)
    }

    async fn publish(
        &self,
        _transaction: &mut publish::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, publish::Error> {
        Ok(context + 2)
    }

    async fn post_publish(
        &self,
        _transaction: &mut publish::transactions::RootTransaction,
        context: Self::Context,
    ) -> Result<Self::Context, publish::Error> {
        Ok(context + 3)
    }
}

#[tokio::test]
async fn test_runner_success() {
    let ctx = 0;

    let test_publish = TestPublish;

    let result = publish::run(ctx, &test_publish).await.unwrap();

    assert_eq!(result, 0 + 1 + 2 + 3);
}
