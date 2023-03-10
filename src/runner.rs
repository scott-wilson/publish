use crate::transactions::Transaction;

pub async fn run<C, P>(context: C, publish: &P) -> Result<C, crate::Error>
where
    P: crate::Publish<Context = C> + Send + Sync,
    C: Clone + Send + Sync,
{
    let mut pre_publish_transaction = crate::transactions::RootTransaction::new();
    let pre_publish_context = match publish
        .pre_publish(&mut pre_publish_transaction, context)
        .await
    {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = pre_publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }
            return Err(err);
        }
    };

    if let Err(commit_err) = pre_publish_transaction.commit().await {
        if let Err(rollback_err) = pre_publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }

        return Err(crate::Error::Commit(Box::new(commit_err)));
    }

    let mut publish_transaction = crate::transactions::RootTransaction::new();
    let publish_context = match publish
        .publish(&mut publish_transaction, pre_publish_context)
        .await
    {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }
            if let Err(rollback_err) = pre_publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            return Err(err);
        }
    };

    if let Err(commit_err) = publish_transaction.commit().await {
        if let Err(rollback_err) = publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }
        if let Err(rollback_err) = pre_publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }

        return Err(crate::Error::Commit(Box::new(commit_err)));
    }

    let mut post_publish_transaction = crate::transactions::RootTransaction::new();
    let post_publish_context = match publish
        .post_publish(&mut post_publish_transaction, publish_context)
        .await
    {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = post_publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }
            if let Err(rollback_err) = publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }
            if let Err(rollback_err) = pre_publish_transaction.rollback().await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            return Err(err);
        }
    };

    if let Err(commit_err) = post_publish_transaction.commit().await {
        if let Err(rollback_err) = post_publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }
        if let Err(rollback_err) = publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }
        if let Err(rollback_err) = pre_publish_transaction.rollback().await {
            return Err(crate::Error::Rollback(
                Box::new(rollback_err),
                Box::new(commit_err),
            ));
        }

        return Err(crate::Error::Commit(Box::new(commit_err)));
    }

    Ok(post_publish_context)
}
