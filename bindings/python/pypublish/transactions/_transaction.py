import abc


class Transaction(abc.ABC):
    """The transaction interface.

    A transaction is the core of the publish framework. All publishes are a
    collection of transactions and transformations to the data. Committed
    transactions may be rolled back if it makes sense to do so. For example,
    if a file is copied, it can be deleted. However, if a file is deleted,
    then it cannot un-deleted.
    """

    @abc.abstractmethod
    async def commit(self) -> None:  # pragma: no cover
        """Commit the transaction."""
        ...

    @abc.abstractmethod
    async def rollback(self) -> None:  # pragma: no cover
        """Rollback the transaction."""
        ...
