from __future__ import annotations

import abc
from typing import TYPE_CHECKING, Generic, TypeVar

if TYPE_CHECKING:  # pragma: no cover
    from .transactions import RootTransaction

C = TypeVar("C")


class Publish(Generic[C], metaclass=abc.ABCMeta):
    """The Publish interface.

    This interface is used to define a publish process such as publishing an
    asset, animation cache, etc.

    The publish shouldn't need to define an :code:`__init__` method, since the
    context should contain all of the information needed to run the publish,
    and optionally the results of each publish stage. Each publish stage may
    transform the data within the context or session, then use the transactions
    to make the transformation permanent. Transactions could represent a
    filesystem update, publishing to a database, etc. Lastly, transactions can
    be rolled back if one of the publish stages fail.
    """

    async def pre_publish(self, transaction: RootTransaction, context: C) -> C:
        """Pre-publish stage.

        This stage should be used to prepare the main publish. For example,
        creating and unlocking the publish directory, preparing a publish
        database entry, etc.

        Args:
            transaction: The collections of transactions to run for the
                pre-publish stage.
            context: The context to use for the pre-publish stage.

        Returns:
            The context and results of the pre-publish stage.
        """
        return context

    @abc.abstractmethod
    async def publish(self, transaction: RootTransaction, context: C) -> C:
        """Publish stage.

        This stage should be used for the main publish work. For example,
        generating caches, transforming rigs into an optimized version, etc.
        Then, the publish stage should use the transactions to make the changes
        permanent.

        Args:
            transaction: The collections of transactions to run for the
                publish stage.
            context: The context to use for the publish stage.

        Returns:
            The context and results of the publish stage.
        """
        ...  # pragma: no cover

    async def post_publish(self, transaction: RootTransaction, context: C) -> C:
        """Post-publish stage.

        This stage should be used to finalize the publish. For example,
        generating a metadata file that contains data about the files in the
        publish such as a checksum, stats, etc. Or, finalizing the publish
        database entry.

        Args:
            transaction: The collections of transactions to run for the
                post-publish stage.
            context: The context to use for the post-publish stage.

        Returns:
            The context and results of the post-publish stage.
        """
        return context
