from __future__ import annotations

from typing import TYPE_CHECKING, Any

import pytest
import hypothesis
from hypothesis import strategies

import pypublish
import pytest

if TYPE_CHECKING:  # pragma: no cover
    pass

# ruff: noqa: S101

pytestmark = pytest.mark.asyncio


class CommitFailTransaction(pypublish.transactions.Transaction):
    def value(self) -> None:
        return None

    async def commit(self) -> None:
        raise RuntimeError("Commit failed")

    async def rollback(self) -> None:
        pass


class RollbackFailTransaction(pypublish.transactions.Transaction):
    def value(self) -> None:
        return None

    async def commit(self) -> None:
        pass

    async def rollback(self) -> None:
        raise RuntimeError("Rollback failed")


async def test_run_success():
    transaction_values = []

    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self, value: int) -> None:
            self.values = transaction_values
            self.__value = value

        def value(self) -> int:
            return self.__value

        async def commit(self) -> None:
            self.values.append(self.__value)

        async def rollback(self) -> None:
            self.values.pop()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            txn = TestTransaction(1)
            transaction.add_child(txn)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            txn = TestTransaction(2)
            transaction.add_child(txn)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            txn = TestTransaction(3)
            transaction.add_child(txn)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()
    result = await pypublish.run(ctx, test_publish)

    assert result == 1 + 2 + 3
    assert test_publish.values == [
        ("pre_publish", 1),
        ("publish", 3),
        ("post_publish", 6),
    ]
    assert transaction_values == [1, 2, 3]


async def test_run_without_pre_post_publish_success():
    transaction_values = []

    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self, value: int) -> None:
            self.values = transaction_values
            self.__value = value

        def value(self) -> int:
            return self.__value

        async def commit(self) -> None:
            self.values.append(self.__value)

        async def rollback(self) -> None:
            self.values.pop()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            txn = TestTransaction(2)
            transaction.add_child(txn)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()
    result = await pypublish.run(ctx, test_publish)

    assert result == 2
    assert test_publish.values == [
        ("publish", 2),
    ]
    assert transaction_values == [2]


async def test_run_failure_prepublish_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            raise RuntimeError("Prepublish failed")

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 2
            transaction.add_child(test_transaction)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 3
            transaction.add_child(test_transaction)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Prepublish failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == []
    assert test_transaction.values == []


async def test_run_failure_publish_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 1
            transaction.add_child(test_transaction)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            raise RuntimeError("Publish failed")

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 3
            transaction.add_child(test_transaction)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Publish failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1)]
    assert test_transaction.values == []


async def test_run_failure_postpublish_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 1
            transaction.add_child(test_transaction)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 2
            transaction.add_child(test_transaction)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            raise RuntimeError("Postpublish failed")

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Postpublish failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1), ("publish", 3)]
    assert test_transaction.values == []


async def test_run_failure_prepublish_rollback_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(RollbackFailTransaction())
            raise RuntimeError("Prepublish failed")

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 2
            transaction.add_child(test_transaction)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 3
            transaction.add_child(test_transaction)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Rollback failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == []
    assert test_transaction.values == []


async def test_run_failure_publish_rollback_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 1
            transaction.add_child(test_transaction)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(RollbackFailTransaction())
            raise RuntimeError("Publish failed")

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 3
            transaction.add_child(test_transaction)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Rollback failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1)]
    assert test_transaction.values == []


async def test_run_failure_postpublish_rollback_fail():
    class TestTransaction(pypublish.transactions.Transaction):
        def __init__(self) -> None:
            self.values = []
            self._value = 0

        def value(self) -> int:
            return self._value

        async def commit(self) -> None:
            self.values.append(self._value)

        async def rollback(self) -> None:
            self.values.pop()

    test_transaction = TestTransaction()

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 1
            transaction.add_child(test_transaction)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            test_transaction._value = 2
            transaction.add_child(test_transaction)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(RollbackFailTransaction())
            raise RuntimeError("Postpublish failed")

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Rollback failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1), ("publish", 3)]
    assert test_transaction.values == []


async def test_run_failure_prepublish_commit_failure():
    transaction_values = []

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(CommitFailTransaction())

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Commit failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1)]


async def test_run_failure_publish_commit_failure():
    transaction_values = []

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(CommitFailTransaction())

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Commit failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [("pre_publish", 1), ("publish", 3)]


async def test_run_failure_postpublish_commit_failure():
    transaction_values = []

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            transaction.add_child(CommitFailTransaction())

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    with pytest.raises(RuntimeError, match="Commit failed"):
        await pypublish.run(ctx, test_publish)

    assert test_publish.values == [
        ("pre_publish", 1),
        ("publish", 3),
        ("post_publish", 6),
    ]


@pytest.mark.parametrize(
    "raise_prepublish", [True, False], ids=["prepublish_err", "prepublish_ok"]
)
@pytest.mark.parametrize(
    "raise_publish", [True, False], ids=["publish_err", "publish_ok"]
)
@pytest.mark.parametrize(
    "raise_postpublish", [True, False], ids=["postpublish_err", "postpublish_ok"]
)
@pytest.mark.parametrize(
    "raise_prepublish_commit",
    [True, False],
    ids=["prepublish_commit_err", "prepublish_commit_ok"],
)
@pytest.mark.parametrize(
    "raise_publish_commit",
    [True, False],
    ids=["publish_commit_err", "publish_commit_ok"],
)
@pytest.mark.parametrize(
    "raise_postpublish_commit",
    [True, False],
    ids=["postpublish_commit_err", "postpublish_commit_ok"],
)
@pytest.mark.parametrize(
    "raise_prepublish_rollback",
    [True, False],
    ids=["prepublish_rollback_err", "prepublish_rollback_ok"],
)
@pytest.mark.parametrize(
    "raise_publish_rollback",
    [True, False],
    ids=["publish_rollback_err", "publish_rollback_ok"],
)
@pytest.mark.parametrize(
    "raise_postpublish_rollback",
    [True, False],
    ids=["postpublish_rollback_err", "postpublish_rollback_ok"],
)
async def test_run_failure_step_raised_exception(
    raise_prepublish: bool,
    raise_publish: bool,
    raise_postpublish: bool,
    raise_prepublish_commit: bool,
    raise_publish_commit: bool,
    raise_postpublish_commit: bool,
    raise_prepublish_rollback: bool,
    raise_publish_rollback: bool,
    raise_postpublish_rollback: bool,
):
    class TestPrePublishTransaction(pypublish.transactions.Transaction):
        def value(self) -> None:
            return None

        async def commit(self) -> None:
            if raise_prepublish_commit:
                raise RuntimeError("Commit failed")

        async def rollback(self) -> None:
            if raise_prepublish_rollback:
                raise RuntimeError("Rollback failed")

    class TestPublishTransaction(pypublish.transactions.Transaction):
        def value(self) -> None:
            return None

        async def commit(self) -> None:
            if raise_publish_commit:
                raise RuntimeError("Commit failed")

        async def rollback(self) -> None:
            if raise_publish_rollback:
                raise RuntimeError("Rollback failed")

    class TestPostPublishTransaction(pypublish.transactions.Transaction):
        def value(self) -> None:
            return None

        async def commit(self) -> None:
            if raise_postpublish_commit:
                raise RuntimeError("Commit failed")

        async def rollback(self) -> None:
            if raise_postpublish_rollback:
                raise RuntimeError("Rollback failed")

    class TestPublish(pypublish.Publish):
        def __init__(self) -> None:
            self.values = []

        async def pre_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            if raise_prepublish:
                raise RuntimeError("Prepublish failed")

            txn = TestPrePublishTransaction()
            transaction.add_child(txn)

            ctx = context + 1
            self.values.append(("pre_publish", ctx))
            return ctx

        async def publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            if raise_publish:
                raise RuntimeError("Publish failed")

            txn = TestPublishTransaction()
            transaction.add_child(txn)

            ctx = context + 2
            self.values.append(("publish", ctx))
            return ctx

        async def post_publish(
            self, transaction: pypublish.transactions.RootTransaction, context: int
        ) -> int:
            if raise_postpublish:
                raise RuntimeError("Postpublish failed")

            txn = TestPostPublishTransaction()
            transaction.add_child(txn)

            ctx = context + 3
            self.values.append(("post_publish", ctx))
            return ctx

    ctx = 0
    test_publish = TestPublish()

    if not any(
        [
            raise_prepublish,
            raise_publish,
            raise_postpublish,
            raise_prepublish_commit,
            raise_publish_commit,
            raise_postpublish_commit,
            raise_prepublish_rollback,
            raise_publish_rollback,
            raise_postpublish_rollback,
        ]
    ) or not any(
        [
            raise_prepublish,
            raise_publish,
            raise_postpublish,
            raise_prepublish_commit,
            raise_publish_commit,
            raise_postpublish_commit,
        ]
    ):
        # Should not raise an exception, since transaction is not rolled back,
        # or rollback is successful
        await pypublish.run(ctx, test_publish)
        return

    with pytest.raises(RuntimeError):
        await pypublish.run(ctx, test_publish)
