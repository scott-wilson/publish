/// The Publish interface.
///
/// This interface is used to define a publish process such as publishing an
/// asset, animation cache, etc.
///
/// The publish struct should not need any attributes or initialization methods,
/// since the context should contain all of the information needed to run the
/// publish, and optionally the results of each publish stage. Each publish
/// stage may transform the data within the context or session, then use the
/// transactions to make the transformation permanent. Transactions could
/// represent a filesystem update, publishing to a database, etc. Lastly,
/// transactions can be rolled back if one of the publish stages fail.
#[async_trait::async_trait]
pub trait Publish {
    /// Pre-publish stage.
    ///
    /// This stage should be used to prepare the main publish. For example,
    /// creating and unlocking the publish directory, preparing a publish
    /// database entry, etc.
    async fn pre_publish<'a>(
        &self,
        context: &'a crate::Context,
    ) -> Result<std::borrow::Cow<'a, crate::Context>, crate::Error> {
        Ok(std::borrow::Cow::Borrowed(context))
    }

    async fn rollback_pre_publish(&self, _context: &crate::Context) -> Result<(), crate::Error> {
        Ok(())
    }

    /// Publish stage.
    ///
    /// This stage should be used for the main publish work. For example,
    /// generating caches, transforming rigs into an optimized version, etc.
    /// Then, the publish stage should use the transactions to make the changes
    /// permanent.
    async fn publish<'a>(
        &self,
        context: &'a crate::Context,
    ) -> Result<std::borrow::Cow<'a, crate::Context>, crate::Error>;

    async fn rollback_publish(&self, _context: &crate::Context) -> Result<(), crate::Error> {
        Ok(())
    }

    /// Post-publish stage.
    ///
    /// This stage should be used to finalize the publish. For example,
    /// generating a metadata file that contains data about the files in the
    /// publish such as a checksum, stats, etc. Or, finalizing the publish
    /// database entry.
    async fn post_publish<'a>(
        &self,
        context: &'a crate::Context,
    ) -> Result<std::borrow::Cow<'a, crate::Context>, crate::Error> {
        Ok(std::borrow::Cow::Borrowed(context))
    }

    async fn rollback_post_publish(&self, _context: &crate::Context) -> Result<(), crate::Error> {
        Ok(())
    }
}
