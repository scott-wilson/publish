/// Run a publish in a given context.
///
/// If the run function fails, then it will automatically roll back all of the
/// stages that have been run.
pub async fn run<P>(publish: &P) -> Result<crate::Context, crate::Error>
where
    P: crate::Publish + Send + Sync,
{
    let context = crate::Context::new();
    let pre_publish_context = match publish.pre_publish(&context).await {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = publish.rollback_pre_publish(&context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            return Err(err);
        }
    };

    let publish_context = match publish.publish(&pre_publish_context).await {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = publish.rollback_publish(&pre_publish_context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            if let Err(rollback_err) = publish.rollback_pre_publish(&context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            return Err(err);
        }
    };

    let post_publish_context = match publish.post_publish(&publish_context).await {
        Ok(ctx) => ctx,
        Err(err) => {
            if let Err(rollback_err) = publish.rollback_post_publish(&publish_context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }
            if let Err(rollback_err) = publish.rollback_publish(&pre_publish_context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            if let Err(rollback_err) = publish.rollback_pre_publish(&context).await {
                return Err(crate::Error::Rollback(
                    Box::new(rollback_err),
                    Box::new(err),
                ));
            }

            return Err(err);
        }
    };

    Ok(post_publish_context.into_owned())
}
