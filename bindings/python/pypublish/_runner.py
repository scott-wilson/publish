from __future__ import annotations

from typing import TYPE_CHECKING, TypeVar

if TYPE_CHECKING:  # pragma: no cover
    from pypublish import Publish

C = TypeVar("C")


async def run(context: C, publish: Publish[C]) -> C:
    """Run a publish in a given context.

    If the run function fails, then it will automatically roll back all of the
    stages that have been run.

    Args:
        context: The context to use for the publish. Depending on the publish,
            this might contain the pointer to the asset, publish notes, etc.
        publish: The publish type to run.

    Raises:
        Exception: If the publish fails, then this will attempt to roll back
            the stages that have been run, and raise the exception that caused
            the error. However, if the rollback fails, then this will raise the
            rollback's error instead.

    Returns:
        The results of the publish. This might contain the path to the publish
            or the publish database entry.
    """
    from pypublish.transactions import RootTransaction

    pre_publish_transaction = RootTransaction()

    try:
        pre_publish_context = await publish.pre_publish(
            pre_publish_transaction, context
        )
    except Exception as pre_publish_err:
        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            raise rollback_err

        raise pre_publish_err

    try:
        await pre_publish_transaction.commit()
    except Exception as commit_err:
        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            raise rollback_err

        raise commit_err

    publish_transaction = RootTransaction()
    try:
        publish_context = await publish.publish(
            publish_transaction, pre_publish_context
        )
    except Exception as publish_err:
        last_rollback_err = None
        try:
            await publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        if last_rollback_err:
            raise last_rollback_err

        raise publish_err

    try:
        await publish_transaction.commit()
    except Exception as commit_err:
        last_rollback_err = None

        try:
            await publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        if last_rollback_err:
            raise last_rollback_err

        raise commit_err

    post_publish_transaction = RootTransaction()

    try:
        post_publish_context = await publish.post_publish(
            post_publish_transaction, publish_context
        )
    except Exception as post_publish_err:
        last_rollback_err = None

        try:
            await post_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        if last_rollback_err:
            raise last_rollback_err

        raise post_publish_err

    try:
        await post_publish_transaction.commit()
    except Exception as commit_err:
        last_rollback_err = None

        try:
            await post_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        try:
            await pre_publish_transaction.rollback()
        except Exception as rollback_err:  # noqa: BLE001
            # TODO: Make a comment about what was the commit error when the rollback
            # error happened.
            last_rollback_err = rollback_err

        if last_rollback_err:
            raise last_rollback_err

        raise commit_err

    return post_publish_context
