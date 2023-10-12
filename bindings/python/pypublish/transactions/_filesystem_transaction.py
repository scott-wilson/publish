from __future__ import annotations

import dataclasses
import enum
import pathlib
import shutil
import sys
from typing import TYPE_CHECKING, Union

import anyio
import anyio.abc

from ._transaction import Transaction

if TYPE_CHECKING:  # pragma: no cover
    from os import PathLike
    from typing import List, Optional


class UnixPermissions(enum.IntFlag):
    UserRead = 0o400
    UserWrite = 0o200
    UserExecute = 0o100
    GroupRead = 0o040
    GroupWrite = 0o020
    GroupExecute = 0o010
    OtherRead = 0o004
    OtherWrite = 0o002
    OtherExecute = 0o001


class Permission(enum.Enum):
    """The permission enum represents the state of a permission bit."""

    Unchanged = enum.auto()
    """Unchanged means that the permission bit should not be changed.

    For example, if the read permission should be set or unset, but the write
    and execute should be unchanged, then the permissions as a tuple would look
    like :code:`(Permission.Set, Permission.Unchanged, Permission.Unchanged)`.
    """
    Set = enum.auto()
    """Set represents that the permission bit should be set to enabled."""
    Unset = enum.auto()
    """Unset represents that the permission bit should be set to disabled."""

    def overwrite(self, other: Permission) -> Permission:
        """Return a new permission with the other permission applied.

        If the other permission is :code:`Permission.Unchanged`, then the
        original permission will be used instead.

        Args:
            other: The other permission to apply.

        Returns:
            The new permission.
        """
        if other == Permission.Unchanged:
            return self
        else:
            return other


@dataclasses.dataclass
class ScopedPermission:
    """The ScopedPermission represents a permission set for an entity.

    This would be the read, write, or execute permissions for a user, group, or
    other.

    Args:
        read: The read permission. Defaults to :code:`Permission.Unchanged`.
        write: The write permission. Defaults to :code:`Permission.Unchanged`.
        execute: The execute permission. Defaults to
            :code:`Permission.Unchanged`.
    """

    read: Permission
    """The read permission."""
    write: Permission
    """The write permission."""
    execute: Permission
    """The execute permission."""

    def __init__(
        self,
        read: Optional[Permission] = None,
        write: Optional[Permission] = None,
        execute: Optional[Permission] = None,
    ) -> None:
        if read is None:
            self.read = Permission.Unchanged
        else:
            self.read = read

        if write is None:
            self.write = Permission.Unchanged
        else:
            self.write = write

        if execute is None:
            self.execute = Permission.Unchanged
        else:
            self.execute = execute

    def overwrite(self, other: ScopedPermission) -> ScopedPermission:
        """Return a new permission with the other permission applied.

        If the other permission is :code:`Permission.Unchanged`, then the
        original permission will be used instead.

        Args:
            other: The other permission to apply.

        Returns:
            The new scoped permission.
        """
        return ScopedPermission(
            read=self.read.overwrite(other.read),
            write=self.write.overwrite(other.write),
            execute=self.execute.overwrite(other.execute),
        )


@dataclasses.dataclass
class Permissions:
    """The Permissions type represents a full set of permissions.

    This contains the read, write, and execute permissions for users, groups,
    and other. This assumes a Unix-like file system.

    Args:
        user: The user permissions. Defaults unchanged permissions.
        group: The group permissions. Defaults unchanged permissions.
        other: The other permissions. Defaults unchanged permissions.
    """

    user: ScopedPermission
    """The user permissions."""
    group: ScopedPermission
    """The group permissions."""
    other: ScopedPermission
    """The other permissions."""

    def __init__(
        self,
        user: Optional[ScopedPermission] = None,
        group: Optional[ScopedPermission] = None,
        other: Optional[ScopedPermission] = None,
    ) -> None:
        if user is None:
            self.user = ScopedPermission()
        else:
            self.user = user

        if group is None:
            self.group = ScopedPermission()
        else:
            self.group = group

        if other is None:
            self.other = ScopedPermission()
        else:
            self.other = other

    def overwrite(self, other: Permissions) -> Permissions:
        """Return a new permission with the other permission applied.

        If the other permission is :code:`Permission.Unchanged`, then the
        original permission will be used instead.

        Args:
            other: The other permission to apply.

        Returns:
            The new permission.
        """
        return Permissions(
            user=self.user.overwrite(other.user),
            group=self.group.overwrite(other.group),
            other=self.other.overwrite(other.other),
        )

    @classmethod
    def from_mode(cls, mode: int) -> Permissions:
        """Convert a Unix mode to a Permissions object.

        This will ignore any bits that are not related to the user, group, or
        other permissions.

        Args:
            mode: The Unix mode to convert.

        Returns:
            The converted permissions.
        """
        unix_mode = UnixPermissions(mode)

        user_read = (
            Permission.Set
            if UnixPermissions.UserRead in unix_mode
            else Permission.Unset
        )
        user_write = (
            Permission.Set
            if UnixPermissions.UserWrite in unix_mode
            else Permission.Unset
        )
        user_execute = (
            Permission.Set
            if UnixPermissions.UserExecute in unix_mode
            else Permission.Unset
        )
        group_read = (
            Permission.Set
            if UnixPermissions.GroupRead in unix_mode
            else Permission.Unset
        )
        group_write = (
            Permission.Set
            if UnixPermissions.GroupWrite in unix_mode
            else Permission.Unset
        )
        group_execute = (
            Permission.Set
            if UnixPermissions.GroupExecute in unix_mode
            else Permission.Unset
        )
        other_read = (
            Permission.Set
            if UnixPermissions.OtherRead in unix_mode
            else Permission.Unset
        )
        other_write = (
            Permission.Set
            if UnixPermissions.OtherWrite in unix_mode
            else Permission.Unset
        )
        other_execute = (
            Permission.Set
            if UnixPermissions.OtherExecute in unix_mode
            else Permission.Unset
        )

        user = ScopedPermission(user_read, user_write, user_execute)
        group = ScopedPermission(group_read, group_write, group_execute)
        other = ScopedPermission(other_read, other_write, other_execute)

        return cls(user, group, other)

    def as_mode(self) -> int:
        """Convert the permissions to a Unix mode integer.

        Returns:
            The Unix mode.
        """
        mode = 0

        if self.user.read == Permission.Set:
            mode |= UnixPermissions.UserRead
        if self.user.write == Permission.Set:
            mode |= UnixPermissions.UserWrite
        if self.user.execute == Permission.Set:
            mode |= UnixPermissions.UserExecute
        if self.group.read == Permission.Set:
            mode |= UnixPermissions.GroupRead
        if self.group.write == Permission.Set:
            mode |= UnixPermissions.GroupWrite
        if self.group.execute == Permission.Set:
            mode |= UnixPermissions.GroupExecute
        if self.other.read == Permission.Set:
            mode |= UnixPermissions.OtherRead
        if self.other.write == Permission.Set:
            mode |= UnixPermissions.OtherWrite
        if self.other.execute == Permission.Set:
            mode |= UnixPermissions.OtherExecute

        return mode


class FilesystemTransactionType(enum.Enum):
    Invalid = enum.auto()
    ChangedOwnerPermissions = enum.auto()
    CreateDirectory = enum.auto()
    CopyMoveHardLink = enum.auto()
    SoftLink = enum.auto()
    Delete = enum.auto()


@dataclasses.dataclass
class CopyAction:
    source: anyio.Path
    target: anyio.Path


@dataclasses.dataclass
class MoveAction:
    source: anyio.Path
    target: anyio.Path


@dataclasses.dataclass
class RollbackMoveAction:
    source: anyio.Path
    target: anyio.Path


@dataclasses.dataclass
class SoftLinkAction:
    source: anyio.Path
    target: anyio.Path


@dataclasses.dataclass
class HardLinkAction:
    source: anyio.Path
    target: anyio.Path


@dataclasses.dataclass
class CreateDirectoryAction:
    path: anyio.Path


@dataclasses.dataclass
class DeleteAction:
    path: anyio.Path


@dataclasses.dataclass
class ChangedOwnerPermissionsAction:
    path: anyio.Path
    user: Optional[str]
    group: Optional[str]
    permissions: Optional[Permissions]


FileSystemAction = Union[
    CopyAction,
    MoveAction,
    RollbackMoveAction,
    SoftLinkAction,
    HardLinkAction,
    CreateDirectoryAction,
    DeleteAction,
    ChangedOwnerPermissionsAction,
]


class FilesystemTransaction(Transaction):
    """The FilesystemTransaction type handles filesystem operations.

    This includes copying paths, moving paths, hard linking paths, soft linking
    paths, deleting paths, creating directories, and changing permissions and
    ownership.

    All of the actions except deleting can be rolled back. The permission and
    ownership metadata will be rolled back to at the point of the transaction
    commit.

    Actions that can be grouped together will be run in parallel. For example,
    copying, moving, or hard linking multiple paths will be done in parallel.
    However, this also depends on the order of actions sent to the transaction.
    So, if you copy a path, delete a path, then copy another path, then all of
    the actions will be done one after another. However, if you copy a path,
    copy another path, then delete a path, then the copy actions will be done
    in parallel before the delete action.

    Args:
        root_dir: The root directory of the transaction. This is used to
            control what happens in the directory. For example, if a copy goes
            from an arbitrary path to another arbitrary path that is not a
            subpath of the root, then the copy will fail. Each of the actions
            will state which argument must be relative to the root or not.
    """

    def __init__(self, root_dir: PathLike[str]) -> None:
        self.__root_dir = anyio.Path(root_dir)
        self.__commit_actions: List[FileSystemAction] = []
        self.__rollback_actions: List[FileSystemAction] = []

    def value(self) -> Optional[List[FileSystemAction]]:
        return self.__commit_actions

    def copy_path(self, source: PathLike[str], target: PathLike[str]) -> None:
        """Copy a path to the root folder.

        Rolling back this action will delete the target path.

        Args:
            source: The source path. This may not be relative to the root.
            target: The target path. This must be relative to the root.
        """
        self.__commit_actions.append(CopyAction(anyio.Path(source), anyio.Path(target)))

    def move_path(self, source: PathLike[str], target: PathLike[str]) -> None:
        """Move a path to the root folder.

        Rolling back this action will move the path back to its original
        location.

        Args:
            source: The source path. This may not be relative to the root.
            target: The target path. This must be relative to the root.
        """
        self.__commit_actions.append(MoveAction(anyio.Path(source), anyio.Path(target)))

    def hard_link_path(self, source: PathLike[str], target: PathLike[str]) -> None:
        """Hard link a path to the root folder.

        Rolling back this action will delete the target path.

        The source and target paths are relative to the root because this
        assumes that each root directory is a self-contained and immutable
        package. Any file that is outside of the root directory is assumed to
        be mutable.

        Args:
            source: The source path. This must be relative to the root.
            target: The target path. This must be relative to the root.
        """
        self.__commit_actions.append(
            HardLinkAction(anyio.Path(source), anyio.Path(target))
        )

    def soft_link_path(self, source: PathLike[str], target: PathLike[str]) -> None:
        """Soft link a path to the root folder.

        Rolling back this action will delete the target path.

        The source and target paths are relative to the root because this
        assumes that each root directory is a self-contained and immutable
        package. Any path that is outside of the root directory is assumed to
        be mutable.

        Args:
            source: The source path. This must be relative to the root.
            target: The target path. This must be relative to the root.
        """
        self.__commit_actions.append(
            SoftLinkAction(anyio.Path(source), anyio.Path(target))
        )

    def change_owner_permissions(
        self,
        path: PathLike[str],
        user: Optional[str] = None,
        group: Optional[str] = None,
        permissions: Optional[Permissions] = None,
    ) -> None:
        """Change the owner and permissions of a path.

        Rolling back this action will change the owner and permissions back to
        what they were before the transaction was committed.

        Args:
            path: The path to change the owner and permissions of. This must
                be relative to the root.
            user: The user to change the owner to. If None, then the owner is
                not changed.
            group: The group to change the owner to. If None, then the owner is
                not changed.
            permissions: The permissions to change the path to. If None, then
                the permissions are not changed.
        """
        self.__commit_actions.append(
            ChangedOwnerPermissionsAction(anyio.Path(path), user, group, permissions)
        )

    def create_directory(self, path: PathLike[str]) -> None:
        """Create a directory.

        Rolling back this action will delete the directory.

        Args:
            path: The path to create the directory at. This must be relative
                to the root.
        """
        self.__commit_actions.append(CreateDirectoryAction(anyio.Path(path)))

    def delete_path(self, path: PathLike[str]) -> None:
        """Delete a path.

        This action cannot be rolled back.

        Args:
            path: The path to delete. This must be relative to the root.
        """
        self.__commit_actions.append(DeleteAction(anyio.Path(path)))

    async def commit(self) -> None:
        """Commit the transaction."""
        del self.__rollback_actions[:]

        last_action_type = FilesystemTransactionType.Invalid
        buf: List[FileSystemAction] = []

        for action in self.__commit_actions:
            if isinstance(action, CopyAction):
                if last_action_type != FilesystemTransactionType.CopyMoveHardLink:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.CopyMoveHardLink

                buf.append(action)
                self.__rollback_actions.append(DeleteAction(action.target))
            elif isinstance(action, MoveAction):
                if last_action_type != FilesystemTransactionType.CopyMoveHardLink:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.CopyMoveHardLink

                buf.append(action)
                self.__rollback_actions.append(
                    RollbackMoveAction(action.source, action.target)
                )
            elif isinstance(action, HardLinkAction):
                if last_action_type != FilesystemTransactionType.CopyMoveHardLink:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.CopyMoveHardLink

                buf.append(action)
                self.__rollback_actions.append(DeleteAction(action.target))
            elif isinstance(action, SoftLinkAction):
                if last_action_type != FilesystemTransactionType.SoftLink:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.SoftLink

                buf.append(action)
                self.__rollback_actions.append(DeleteAction(action.target))
            elif isinstance(action, CreateDirectoryAction):
                if last_action_type != FilesystemTransactionType.CreateDirectory:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.CreateDirectory

                buf.append(action)
                self.__rollback_actions.append(DeleteAction(action.path))
            elif isinstance(action, DeleteAction):
                if last_action_type != FilesystemTransactionType.Delete:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.Delete

                buf.append(action)
            elif isinstance(action, ChangedOwnerPermissionsAction):
                if (
                    last_action_type
                    != FilesystemTransactionType.ChangedOwnerPermissions
                ):
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.ChangedOwnerPermissions

                buf.append(action)
                self.__rollback_actions.append(
                    await self.__current_file_owner_permissions(action.path)
                )
            else:  # pragma: no cover
                raise RuntimeError(f"Unknown commit action type: {action}")

        if buf:
            await self.__run_actions(buf)

    async def rollback(self) -> None:
        """Rollback the transaction."""
        last_action_type = FilesystemTransactionType.Invalid
        buf: List[FileSystemAction] = []

        for action in self.__rollback_actions:
            if isinstance(action, RollbackMoveAction):
                if last_action_type != FilesystemTransactionType.CopyMoveHardLink:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.CopyMoveHardLink

                buf.append(action)
            elif isinstance(action, DeleteAction):
                if last_action_type != FilesystemTransactionType.Delete:
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.Delete

                buf.append(action)
            elif isinstance(action, ChangedOwnerPermissionsAction):
                if (
                    last_action_type
                    != FilesystemTransactionType.ChangedOwnerPermissions
                ):
                    await self.__run_actions(buf)
                    last_action_type = FilesystemTransactionType.ChangedOwnerPermissions

                buf.append(action)
            else:  # pragma: no cover
                raise RuntimeError(f"Unknown commit action type: {action}")

        if buf:
            await self.__run_actions(buf)

    async def __run_actions(self, actions: List[FileSystemAction]) -> None:
        async with anyio.create_task_group() as task_group:
            for action in actions:
                if isinstance(action, CopyAction):
                    task_group.start_soon(self.__run_copy_action, action)
                elif isinstance(action, MoveAction):
                    task_group.start_soon(self.__run_move_action, action)
                elif isinstance(action, RollbackMoveAction):
                    task_group.start_soon(self.__run_rollback_move_action, action)
                elif isinstance(action, SoftLinkAction):
                    task_group.start_soon(self.__run_soft_link_action, action)
                elif isinstance(action, HardLinkAction):
                    task_group.start_soon(self.__run_hard_link_action, action)
                elif isinstance(action, CreateDirectoryAction):
                    task_group.start_soon(self.__run_create_directory_action, action)
                elif isinstance(action, DeleteAction):
                    task_group.start_soon(self.__run_delete_action, action)
                elif isinstance(action, ChangedOwnerPermissionsAction):
                    task_group.start_soon(
                        self.__run_change_owner_permissions_action, action
                    )
                else:  # pragma: no cover
                    raise ValueError(f"Unknown action type {action}")

        del actions[:]  # Clear the list

    async def __run_copy_action(self, action: CopyAction) -> None:
        source: anyio.Path = action.source
        target: anyio.Path = _validate_path(self.__root_dir, action.target)

        if await source.is_dir():
            await anyio.to_thread.run_sync(
                shutil.copytree, pathlib.Path(source), pathlib.Path(target)
            )
        elif await source.is_file():
            await anyio.to_thread.run_sync(
                shutil.copy, pathlib.Path(source), pathlib.Path(target)
            )
        else:
            raise OSError(f"Path {source} is not a file or directory")

    async def __run_move_action(self, action: MoveAction) -> None:
        source: anyio.Path = action.source
        target: anyio.Path = _validate_path(self.__root_dir, action.target)

        await anyio.to_thread.run_sync(
            shutil.move, pathlib.Path(source), pathlib.Path(target)
        )

    async def __run_rollback_move_action(self, action: RollbackMoveAction) -> None:
        source: anyio.Path = action.source
        target: anyio.Path = _validate_path(self.__root_dir, action.target)

        await anyio.to_thread.run_sync(
            shutil.move, pathlib.Path(target), pathlib.Path(source)
        )

    async def __run_hard_link_action(self, action: HardLinkAction) -> None:
        source: anyio.Path = _validate_path(self.__root_dir, action.source)
        target: anyio.Path = _validate_path(self.__root_dir, action.target)

        await target.hardlink_to(source)

    async def __run_soft_link_action(self, action: SoftLinkAction) -> None:
        source: anyio.Path = _validate_path(self.__root_dir, action.source)
        target: anyio.Path = _validate_path(self.__root_dir, action.target)

        await target.symlink_to(source)

    async def __run_create_directory_action(
        self, action: CreateDirectoryAction
    ) -> None:
        path: anyio.Path = _validate_path(self.__root_dir, action.path)

        await path.mkdir(parents=True)

    async def __run_delete_action(self, action: DeleteAction) -> None:
        path: anyio.Path = _validate_path(self.__root_dir, action.path)

        if await path.is_symlink() or await path.is_file():
            await path.unlink()
        elif await path.is_dir():
            await anyio.to_thread.run_sync(shutil.rmtree, pathlib.Path(path))
        else:
            raise OSError(f"Path {path} is not a file or directory")

    async def __run_change_owner_permissions_action(
        self, action: ChangedOwnerPermissionsAction
    ) -> None:
        path: anyio.Path = _validate_path(self.__root_dir, action.path)

        if action.user or action.group:
            await anyio.to_thread.run_sync(
                shutil.chown, pathlib.Path(path), action.user, action.group
            )
        if action.permissions is not None:
            await path.chmod(action.permissions.as_mode())

    async def __current_file_owner_permissions(
        self, path: anyio.Path
    ) -> ChangedOwnerPermissionsAction:
        abs_path: anyio.Path = _validate_path(self.__root_dir, path)
        stat = await abs_path.stat()

        if sys.platform.startswith("win"):  # pragma: no cover
            raise NotImplementedError("Windows is not implemented yet")
        else:
            import grp
            import pwd

            user = (await anyio.to_thread.run_sync(pwd.getpwuid, stat.st_uid)).pw_name
            group = (await anyio.to_thread.run_sync(grp.getgrgid, stat.st_gid)).gr_name

            return ChangedOwnerPermissionsAction(
                path=path,
                user=user,
                group=group,
                permissions=Permissions.from_mode(stat.st_mode),
            )


def _validate_path(root_dir: anyio.Path, path: anyio.Path) -> anyio.Path:
    abs_path = root_dir / path

    if not abs_path.is_relative_to(root_dir):
        raise PermissionError(f"Path {path} is not relative to {root_dir}")
    elif ".." in abs_path.as_posix():
        raise PermissionError(f"Path {path} is not relative to {root_dir}")

    return abs_path
