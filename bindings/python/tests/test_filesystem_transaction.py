from __future__ import annotations

import pathlib
import re
import stat
import sys
from typing import TYPE_CHECKING

import hypothesis
import pytest
from hypothesis import strategies

import pypublish

if TYPE_CHECKING:  # pragma: no cover
    from typing import Optional

# ruff: noqa: S101


permissions = strategies.sampled_from(pypublish.transactions.Permission)
scoped_permissions = strategies.builds(
    pypublish.transactions.ScopedPermission,
    strategies.none() | permissions,
    strategies.none() | permissions,
    strategies.none() | permissions,
)
file_permissions = strategies.builds(
    pypublish.transactions.Permissions,
    strategies.none() | scoped_permissions,
    strategies.none() | scoped_permissions,
    strategies.none() | scoped_permissions,
)


@pytest.mark.parametrize(
    "permission_a",
    [
        pypublish.transactions.Permission.Unchanged,
        pypublish.transactions.Permission.Set,
        pypublish.transactions.Permission.Unset,
    ],
)
@pytest.mark.parametrize(
    "permission_b",
    [
        pypublish.transactions.Permission.Unchanged,
        pypublish.transactions.Permission.Set,
        pypublish.transactions.Permission.Unset,
    ],
)
def test_permission_overwrite(
    permission_a: pypublish.transactions.Permission,
    permission_b: pypublish.transactions.Permission,
):
    result = permission_a.overwrite(permission_b)

    if permission_b == pypublish.transactions.Permission.Unchanged:
        assert result == permission_a
    else:
        assert result == permission_b


@pytest.mark.parametrize(
    "read_permission",
    [
        None,
        pypublish.transactions.Permission.Unchanged,
        pypublish.transactions.Permission.Set,
        pypublish.transactions.Permission.Unset,
    ],
)
@pytest.mark.parametrize(
    "write_permission",
    [
        None,
        pypublish.transactions.Permission.Unchanged,
        pypublish.transactions.Permission.Set,
        pypublish.transactions.Permission.Unset,
    ],
)
@pytest.mark.parametrize(
    "execute_permission",
    [
        None,
        pypublish.transactions.Permission.Unchanged,
        pypublish.transactions.Permission.Set,
        pypublish.transactions.Permission.Unset,
    ],
)
def test_scoped_permission_init(
    read_permission: Optional[pypublish.transactions.Permission],
    write_permission: Optional[pypublish.transactions.Permission],
    execute_permission: Optional[pypublish.transactions.Permission],
):
    result = pypublish.transactions.ScopedPermission(
        read_permission, write_permission, execute_permission
    )

    if read_permission is None:
        assert result.read == pypublish.transactions.Permission.Unchanged
    else:
        assert result.read == read_permission

    if write_permission is None:
        assert result.write == pypublish.transactions.Permission.Unchanged
    else:
        assert result.write == write_permission

    if execute_permission is None:
        assert result.execute == pypublish.transactions.Permission.Unchanged
    else:
        assert result.execute == execute_permission


@hypothesis.given(permission_a=scoped_permissions, permission_b=scoped_permissions)
def test_scoped_permission_overwrite(
    permission_a: pypublish.transactions.ScopedPermission,
    permission_b: pypublish.transactions.ScopedPermission,
):
    permission_c = permission_a.overwrite(permission_b)

    if permission_b.read == pypublish.transactions.Permission.Unchanged:
        assert permission_c.read == permission_a.read
    else:
        assert permission_c.read == permission_b.read

    if permission_b.write == pypublish.transactions.Permission.Unchanged:
        assert permission_c.write == permission_a.write
    else:
        assert permission_c.write == permission_b.write

    if permission_b.execute == pypublish.transactions.Permission.Unchanged:
        assert permission_c.execute == permission_a.execute
    else:
        assert permission_c.execute == permission_b.execute


@hypothesis.given(
    user=strategies.none() | scoped_permissions,
    group=strategies.none() | scoped_permissions,
    other=strategies.none() | scoped_permissions,
)
def test_file_permissions_init(
    user: pypublish.transactions.ScopedPermission,
    group: pypublish.transactions.ScopedPermission,
    other: pypublish.transactions.ScopedPermission,
):
    result = pypublish.transactions.Permissions(user, group, other)

    if user is None:
        assert result.user == pypublish.transactions.ScopedPermission()
    else:
        assert result.user == user

    if group is None:
        assert result.group == pypublish.transactions.ScopedPermission()
    else:
        assert result.group == group

    if other is None:
        assert result.other == pypublish.transactions.ScopedPermission()
    else:
        assert result.other == other


@hypothesis.given(permission_a=file_permissions, permission_b=file_permissions)
def test_file_permissions_overwrite(
    permission_a: pypublish.transactions.Permissions,
    permission_b: pypublish.transactions.Permissions,
):
    permission_c = permission_a.overwrite(permission_b)

    if permission_b.user.read is pypublish.transactions.Permission.Unchanged:
        assert permission_c.user.read == permission_a.user.read
    else:
        assert permission_c.user.read == permission_b.user.read

    if permission_b.user.write is pypublish.transactions.Permission.Unchanged:
        assert permission_c.user.write == permission_a.user.write
    else:
        assert permission_c.user.write == permission_b.user.write

    if permission_b.user.execute is pypublish.transactions.Permission.Unchanged:
        assert permission_c.user.execute == permission_a.user.execute
    else:
        assert permission_c.user.execute == permission_b.user.execute

    if permission_b.group.read is pypublish.transactions.Permission.Unchanged:
        assert permission_c.group.read == permission_a.group.read
    else:
        assert permission_c.group.read == permission_b.group.read

    if permission_b.group.write is pypublish.transactions.Permission.Unchanged:
        assert permission_c.group.write == permission_a.group.write
    else:
        assert permission_c.group.write == permission_b.group.write

    if permission_b.group.execute is pypublish.transactions.Permission.Unchanged:
        assert permission_c.group.execute == permission_a.group.execute
    else:
        assert permission_c.group.execute == permission_b.group.execute

    if permission_b.other.read is pypublish.transactions.Permission.Unchanged:
        assert permission_c.other.read == permission_a.other.read
    else:
        assert permission_c.other.read == permission_b.other.read

    if permission_b.other.write is pypublish.transactions.Permission.Unchanged:
        assert permission_c.other.write == permission_a.other.write
    else:
        assert permission_c.other.write == permission_b.other.write

    if permission_b.other.execute is pypublish.transactions.Permission.Unchanged:
        assert permission_c.other.execute == permission_a.other.execute
    else:
        assert permission_c.other.execute == permission_b.other.execute


@pytest.mark.parametrize("user_read", [stat.S_IRUSR, 0])
@pytest.mark.parametrize("user_write", [stat.S_IWUSR, 0])
@pytest.mark.parametrize("user_execute", [stat.S_IXUSR, 0])
@pytest.mark.parametrize("group_read", [stat.S_IRGRP, 0])
@pytest.mark.parametrize("group_write", [stat.S_IWGRP, 0])
@pytest.mark.parametrize("group_execute", [stat.S_IXGRP, 0])
@pytest.mark.parametrize("other_read", [stat.S_IROTH, 0])
@pytest.mark.parametrize("other_write", [stat.S_IWOTH, 0])
@pytest.mark.parametrize("other_execute", [stat.S_IXOTH, 0])
def test_file_permissions_from_mode(
    user_read: int,
    user_write: int,
    user_execute: int,
    group_read: int,
    group_write: int,
    group_execute: int,
    other_read: int,
    other_write: int,
    other_execute: int,
):
    mode = (
        user_read
        | user_write
        | user_execute
        | group_read
        | group_write
        | group_execute
        | other_read
        | other_write
        | other_execute
    )
    result = pypublish.transactions.Permissions.from_mode(mode)
    result_map = {
        True: pypublish.transactions.Permission.Set,
        False: pypublish.transactions.Permission.Unset,
    }

    assert result.user.read == result_map[bool(user_read)]
    assert result.user.write == result_map[bool(user_write)]
    assert result.user.execute == result_map[bool(user_execute)]

    assert result.group.read == result_map[bool(group_read)]
    assert result.group.write == result_map[bool(group_write)]
    assert result.group.execute == result_map[bool(group_execute)]

    assert result.other.read == result_map[bool(other_read)]
    assert result.other.write == result_map[bool(other_write)]
    assert result.other.execute == result_map[bool(other_execute)]

    assert result.as_mode() == mode


@pytest.mark.asyncio()
async def test_copy_file_success(tmp_path_factory: pytest.TempPathFactory):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")

    source_file = source_dir / "test"
    source_file.touch()
    target_file = target_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file, target_file.relative_to(target_dir))

    await transaction.commit()

    assert source_file.is_file()
    assert target_file.is_file()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_copy_file_failure_source_does_not_exist(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")

    source_file = source_dir / "test"
    target_file = target_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file, target_file.relative_to(target_dir))

    with pytest.raises(
        OSError, match=f"Path {re.escape(str(source_file))} is not a file or directory"
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_copy_file_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = source_dir / "test"
    source_file.touch()
    target_file = other_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_copy_file_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target") / "target"
    target_dir.mkdir(parents=True)

    source_file = source_dir / "test"
    source_file.touch()
    relative_target_file = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file, relative_target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_copy_dir_success(tmp_path_factory: pytest.TempPathFactory):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = target_dir / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(
        source_file.parent, target_file.parent.relative_to(target_dir)
    )

    await transaction.commit()

    assert source_file.is_file()
    assert target_file.is_file()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_copy_dir_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = other_dir / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file.parent, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_copy_dir_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target") / "target"
    target_dir.mkdir(parents=True)

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = pathlib.Path("..") / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.copy_path(source_file.parent, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_move_file_success(tmp_path_factory: pytest.TempPathFactory):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")

    source_file = source_dir / "test"
    source_file.touch()
    target_file = target_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(source_file, target_file.relative_to(target_dir))

    await transaction.commit()

    assert not source_file.is_file()
    assert target_file.is_file()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_move_file_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = source_dir / "test"
    source_file.touch()
    target_file = other_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(source_file, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_move_file_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target") / "target"
    target_dir.mkdir(parents=True)

    source_file = source_dir / "test"
    source_file.touch()
    relative_target_file = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(source_file, relative_target_file)

    with pytest.raises(
        PermissionError,
        match=f"Path {relative_target_file} is not relative to {target_dir}",
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_move_dir_success(tmp_path_factory: pytest.TempPathFactory):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = target_dir / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(
        source_file.parent, target_file.parent.relative_to(target_dir)
    )

    await transaction.commit()

    assert not source_file.is_file()
    assert target_file.is_file()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_move_dir_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = other_dir / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(source_file.parent, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to"
            f" {re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_move_dir_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    source_dir = tmp_path_factory.mktemp("source")
    target_dir = tmp_path_factory.mktemp("target") / "target"
    target_dir.mkdir(parents=True)

    source_file = source_dir / "test" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = pathlib.Path("..") / "test" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(target_dir)
    transaction.move_path(source_file.parent, target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to "
            f"{re.escape(str(target_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_hard_link_file_success(tmp_path_factory: pytest.TempPathFactory):
    root_dir = tmp_path_factory.mktemp("root")

    source_file = root_dir / "test1"
    source_file.touch()
    target_file = root_dir / "test2"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.hard_link_path(
        source_file.relative_to(root_dir), target_file.relative_to(root_dir)
    )

    await transaction.commit()

    assert source_file.is_file()
    assert target_file.is_file()

    if sys.platform == "linux":
        assert source_file.stat().st_ino == target_file.stat().st_ino

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_hard_link_file_failure_source_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = other_dir / "test"
    source_file.touch()
    target_file = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.hard_link_path(source_file, target_file.relative_to(root_dir))

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(source_file))} is not relative to "
            f"{re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_hard_link_file_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = root_dir / "test"
    source_file.touch()
    target_file = other_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.hard_link_path(source_file.relative_to(root_dir), target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to "
            f"{re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_hard_link_file_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)

    source_file = root_dir / "test"
    source_file.touch()
    relative_target_file = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.hard_link_path(source_file, relative_target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_target_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_hard_link_file_failure_source_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)

    source_file = root_dir.parent / "test"
    source_file.touch()
    relative_source_file = pathlib.Path("..") / "test"
    target_file = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.hard_link_path(relative_source_file, target_file.relative_to(root_dir))

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_source_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_soft_link_file_success(tmp_path_factory: pytest.TempPathFactory):
    root_dir = tmp_path_factory.mktemp("root")

    source_file = root_dir / "test1"
    source_file.touch()
    target_file = root_dir / "test2"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(
        source_file.relative_to(root_dir), target_file.relative_to(root_dir)
    )

    await transaction.commit()

    assert source_file.is_file()
    assert target_file.is_file()
    assert target_file.is_symlink()
    assert source_file == target_file.resolve()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_soft_link_file_failure_source_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = other_dir / "test"
    source_file.touch()
    target_file = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(source_file, target_file.relative_to(root_dir))

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(source_file))} is not relative to "
            f"{re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_soft_link_file_failure_target_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")

    source_file = root_dir / "test"
    source_file.touch()
    target_file = other_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(source_file.relative_to(root_dir), target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(target_file))} is not relative to "
            f"{re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_soft_link_file_failure_target_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)

    source_file = root_dir / "test"
    source_file.touch()
    relative_target_file = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(source_file, relative_target_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_target_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_soft_link_file_failure_source_is_outside_target_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)

    source_file = root_dir.parent / "test"
    source_file.touch()
    relative_source_file = pathlib.Path("..") / "test"
    target_file = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(relative_source_file, target_file.relative_to(root_dir))

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_source_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_soft_link_dir_success(tmp_path_factory: pytest.TempPathFactory):
    root_dir = tmp_path_factory.mktemp("root")

    source_file = root_dir / "test1" / "test"
    source_file.parent.mkdir()
    source_file.touch()
    target_file = root_dir / "test2" / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.soft_link_path(
        source_file.parent.relative_to(root_dir),
        target_file.parent.relative_to(root_dir),
    )

    await transaction.commit()

    assert source_file.is_file()
    assert target_file.is_file()
    assert target_file.parent.is_symlink()
    assert source_file.parent == target_file.parent.resolve()

    await transaction.rollback()

    assert source_file.is_file()
    assert not target_file.is_file()


@pytest.mark.asyncio()
async def test_change_owner_permissions_success(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")

    test_file = root_dir / "test"
    test_file.touch()
    current_stat = test_file.stat()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    permissions = pypublish.transactions.Permissions.from_mode(current_stat.st_mode)
    permissions.user.read = pypublish.transactions.Permission.Set
    permissions.user.write = pypublish.transactions.Permission.Set
    permissions.user.execute = pypublish.transactions.Permission.Set
    permissions.group.read = pypublish.transactions.Permission.Set
    permissions.group.write = pypublish.transactions.Permission.Set
    permissions.group.execute = pypublish.transactions.Permission.Set
    permissions.other.read = pypublish.transactions.Permission.Set
    permissions.other.write = pypublish.transactions.Permission.Set
    permissions.other.execute = pypublish.transactions.Permission.Set

    transaction.change_owner_permissions(
        test_file.relative_to(root_dir),
        None,
        None,
        permissions,
    )

    await transaction.commit()

    assert test_file.stat().st_mode == 0o100777

    await transaction.rollback()

    assert test_file.stat().st_mode == current_stat.st_mode


@pytest.mark.asyncio()
async def test_change_owner_permissions_failure_path_is_outside_root_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)

    test_file = root_dir.parent / "test"
    test_file.touch()
    relative_test_file = pathlib.Path("..") / "test"
    current_stat = test_file.stat()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    permissions = pypublish.transactions.Permissions.from_mode(current_stat.st_mode)
    permissions.user.read = pypublish.transactions.Permission.Set
    permissions.user.write = pypublish.transactions.Permission.Set
    permissions.user.execute = pypublish.transactions.Permission.Set
    permissions.group.read = pypublish.transactions.Permission.Set
    permissions.group.write = pypublish.transactions.Permission.Set
    permissions.group.execute = pypublish.transactions.Permission.Set
    permissions.other.read = pypublish.transactions.Permission.Set
    permissions.other.write = pypublish.transactions.Permission.Set
    permissions.other.execute = pypublish.transactions.Permission.Set

    transaction.change_owner_permissions(
        relative_test_file,
        None,
        None,
        permissions,
    )

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_test_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_change_owner_permissions_failure_path_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")

    test_file = other_dir / "test"
    test_file.touch()
    current_stat = test_file.stat()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    permissions = pypublish.transactions.Permissions.from_mode(current_stat.st_mode)
    permissions.user.read = pypublish.transactions.Permission.Set
    permissions.user.write = pypublish.transactions.Permission.Set
    permissions.user.execute = pypublish.transactions.Permission.Set
    permissions.group.read = pypublish.transactions.Permission.Set
    permissions.group.write = pypublish.transactions.Permission.Set
    permissions.group.execute = pypublish.transactions.Permission.Set
    permissions.other.read = pypublish.transactions.Permission.Set
    permissions.other.write = pypublish.transactions.Permission.Set
    permissions.other.execute = pypublish.transactions.Permission.Set

    transaction.change_owner_permissions(
        test_file,
        None,
        None,
        permissions,
    )

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(test_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.skipif(
    sys.platform != "win32", reason="Testing if Windows is not implemented"
)
@pytest.mark.asyncio()
async def test_change_owner_permissions_failure_windows_not_implemented(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")

    test_file = root_dir / "test"
    test_file.touch()
    current_stat = test_file.stat()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    permissions = pypublish.transactions.Permissions.from_mode(current_stat.st_mode)

    transaction.change_owner_permissions(
        test_file.relative_to(root_dir),
        None,
        None,
        permissions,
    )

    with pytest.raises(NotImplementedError, match="Windows is not implemented yet"):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_create_directory_success(tmp_path_factory: pytest.TempPathFactory):
    root_dir = tmp_path_factory.mktemp("root")
    child_dir = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.create_directory(child_dir.relative_to(root_dir))

    await transaction.commit()

    assert child_dir.is_dir()

    await transaction.rollback()

    assert not child_dir.exists()


@pytest.mark.asyncio()
async def test_create_directory_failure_directory_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")
    child_dir = other_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.create_directory(child_dir)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(child_dir))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_create_directory_failure_directory_is_outside_root_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)
    relative_child_dir = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.create_directory(relative_child_dir)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_child_dir))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_delete_path_success(tmp_path_factory: pytest.TempPathFactory):
    root_dir = tmp_path_factory.mktemp("root")
    child_file = root_dir / "test"
    child_file.touch()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.delete_path(child_file.relative_to(root_dir))

    await transaction.commit()

    assert not child_file.exists()

    await transaction.rollback()

    assert not child_file.exists()


@pytest.mark.asyncio()
async def test_delete_path_failure_path_is_not_relative(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    other_dir = tmp_path_factory.mktemp("other")
    child_file = other_dir / "test"
    child_file.touch()

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.delete_path(child_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(child_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_delete_path_failure_path_is_outside_root_dir(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root") / "root"
    root_dir.mkdir(parents=True)
    child_file = root_dir.parent / "test"
    child_file.touch()
    relative_child_file = pathlib.Path("..") / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.delete_path(relative_child_file)

    with pytest.raises(
        PermissionError,
        match=(
            f"Path {re.escape(str(relative_child_file))} is not relative to"
            f" {re.escape(str(root_dir))}"
        ),
    ):
        await transaction.commit()


@pytest.mark.asyncio()
async def test_delete_path_failure_path_does_not_exist(
    tmp_path_factory: pytest.TempPathFactory,
):
    root_dir = tmp_path_factory.mktemp("root")
    child_file = root_dir / "test"

    transaction = pypublish.transactions.FilesystemTransaction(root_dir)
    transaction.delete_path(child_file.relative_to(root_dir))

    with pytest.raises(
        OSError, match=f"Path {re.escape(str(child_file))} is not a file or directory"
    ):
        await transaction.commit()
