from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import pypublish

if TYPE_CHECKING:  # pragma: no cover
    from typing import List

# ruff: noqa: S101

pytestmark = pytest.mark.asyncio


class MockTransaction(pypublish.transactions.Transaction):
    def __init__(self, values: List[int], value: int) -> None:
        self.values = values
        self._value = value

    def value(self) -> int:
        return self._value

    async def commit(self) -> None:
        self.values.append(self._value)

    async def rollback(self) -> None:
        self.values.remove(self._value)


async def test_add_child_success():
    values = []

    root_transaction = pypublish.transactions.RootTransaction()
    root_transaction.add_child(MockTransaction(values=values, value=1))
    root_transaction.add_child(MockTransaction(values=values, value=2))
    root_transaction.add_child(MockTransaction(values=values, value=3))

    await root_transaction.commit()

    assert values == [1, 2, 3]

    await root_transaction.rollback()

    assert values == []


async def test_add_parallel_success():
    values = []

    root_transaction = pypublish.transactions.RootTransaction()
    root_transaction.add_child_parallel(MockTransaction(values=values, value=1))
    root_transaction.add_child_parallel(MockTransaction(values=values, value=2))
    root_transaction.add_child_parallel(MockTransaction(values=values, value=3))

    await root_transaction.commit()

    assert sorted(values) == [1, 2, 3]

    await root_transaction.rollback()

    assert values == []
