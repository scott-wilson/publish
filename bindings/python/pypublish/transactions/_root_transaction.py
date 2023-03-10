from __future__ import annotations

from typing import TYPE_CHECKING

import anyio

from ._transaction import Transaction

if TYPE_CHECKING:  # pragma: no cover
    from typing import List


class RootTransaction(Transaction):
    """The RootTransaction lets transactions be run in a hierarchical manner.

    This can be used to run child transactions in parallel, or in sequence.
    Adding a child without using the :code:`add_child_parallel` method will
    cause the child to be run sequentially. Adding a child using the
    :code:`add_child_parallel` will cause the child to be run in parallel with
    either the last child added with :code:`add_child` or the children in
    the last call to :code:`add_child_parallel`.
    """

    def __init__(self) -> None:
        self.__children: List[List[Transaction]] = []

    def add_child(self, transaction: Transaction) -> None:
        """Add a child transaction.

        Args:
            transaction: The transaction to add. This transaction will be run
                sequentially.
        """
        self.__children.append([transaction])

    def add_child_parallel(self, transaction: Transaction) -> None:
        """Add a child transaction to be run in parallel.

        Args:
            transaction: The transaction to add. This transaction will be run
                in parallel with the last child added with :code:`add_child`
                the children in the last call to :code:`add_child_parallel`.
        """
        if not self.__children:
            self.__children.append([])

        self.__children[-1].append(transaction)

    async def commit(self) -> None:
        """Commit the transaction."""
        for children in self.__children:
            async with anyio.create_task_group() as task_group:
                for child in children:
                    task_group.start_soon(child.commit)

    async def rollback(self) -> None:
        """Rollback the transaction."""
        for children in self.__children:
            async with anyio.create_task_group() as task_group:
                for child in children:
                    task_group.start_soon(child.rollback)
