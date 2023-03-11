use std::path::{Path, PathBuf};

bitflags::bitflags! {
    struct UnixPermissions: u32 {
        const USER_READ = 0o400;
        const USER_WRITE = 0o200;
        const USER_EXECUTE = 0o100;
        const GROUP_READ = 0o040;
        const GROUP_WRITE = 0o020;
        const GROUP_EXECUTE = 0o010;
        const OTHER_READ = 0o004;
        const OTHER_WRITE = 0o002;
        const OTHER_EXECUTE = 0o001;
    }
}

/// The permission enum represents the state of a permission bit.
#[derive(Debug, Clone, Copy)]
pub enum Permission {
    /// Unchanged means that the permission bit should not be changed.
    ///
    /// For example, if the read permission should be set or unset, but the
    /// write and execute should be unchanged, then the permissions as a tuple
    /// would look like
    /// `(Permission::Set, Permission::Unchanged, Permission::Unchanged)`
    Unchanged,
    /// Set represents that the permission bit should be set to enabled.
    Set,
    /// Unset represents that the permission bit should be set to disabled.
    Unset,
}

impl Permission {
    /// Return a new permission with the other permission applied.
    ///
    /// If the other permission is [Permission::Unchanged], then the original
    /// permission will be used instead.
    pub fn overwrite(&self, other: &Self) -> Self {
        match other {
            Self::Unchanged => *self,
            Self::Set => Self::Set,
            Self::Unset => Self::Unset,
        }
    }
}

impl Default for Permission {
    fn default() -> Self {
        Self::Unchanged
    }
}

/// The ScopedPermission represents a permission set for an entity.
///
/// This would be the read, write, or execute permissions for a user, group, or
/// other.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScopedPermissions {
    /// The read permission.
    pub read: Permission,
    /// The write permission.
    pub write: Permission,
    /// The execute permission.
    pub execute: Permission,
}

impl ScopedPermissions {
    /// Return a new permission with the other permission applied.
    ///
    /// If the other permission is [Permission::Unchanged], then the original
    /// permission will be used instead.
    pub fn overwrite(&self, other: &Self) -> Self {
        Self {
            read: self.read.overwrite(&other.read),
            write: self.write.overwrite(&other.write),
            execute: self.execute.overwrite(&other.execute),
        }
    }
}

/// The Permissions type represents a full set of permissions.
///
/// This contains the read, write, and execute permissions for users, groups,
/// and other. This assumes a Unix-like file system.
#[derive(Debug, Clone, Copy, Default)]
pub struct Permissions {
    /// The user permissions.
    pub user: ScopedPermissions,
    /// The group permissions.
    pub group: ScopedPermissions,
    /// The other permissions.
    pub other: ScopedPermissions,
}

impl Permissions {
    /// Convert a Unix mode to a Permissions object.
    ///
    /// This will ignore any bits that are not related to the user, group, or
    /// other permissions.
    pub fn from_mode(mode: u32) -> Self {
        let mode = UnixPermissions::from_bits_truncate(mode);

        let user_read = if mode.contains(UnixPermissions::USER_READ) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let user_write = if mode.contains(UnixPermissions::USER_WRITE) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let user_execute = if mode.contains(UnixPermissions::USER_EXECUTE) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let group_read = if mode.contains(UnixPermissions::GROUP_READ) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let group_write = if mode.contains(UnixPermissions::GROUP_WRITE) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let group_execute = if mode.contains(UnixPermissions::GROUP_EXECUTE) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let other_read = if mode.contains(UnixPermissions::OTHER_READ) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let other_write = if mode.contains(UnixPermissions::OTHER_WRITE) {
            Permission::Set
        } else {
            Permission::Unset
        };
        let other_execute = if mode.contains(UnixPermissions::OTHER_EXECUTE) {
            Permission::Set
        } else {
            Permission::Unset
        };

        let user = ScopedPermissions {
            read: user_read,
            write: user_write,
            execute: user_execute,
        };
        let group = ScopedPermissions {
            read: group_read,
            write: group_write,
            execute: group_execute,
        };
        let other = ScopedPermissions {
            read: other_read,
            write: other_write,
            execute: other_execute,
        };

        Permissions { user, group, other }
    }

    /// Convert the permissions to a Unix mode integer.
    pub fn as_mode(&self) -> Result<u32, crate::Error> {
        let mut mode = UnixPermissions::empty();

        match self.user.read {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::USER_READ),
            Permission::Unset => mode.remove(UnixPermissions::USER_READ),
        };
        match self.user.write {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::USER_WRITE),
            Permission::Unset => mode.remove(UnixPermissions::USER_WRITE),
        };
        match self.user.execute {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::USER_EXECUTE),
            Permission::Unset => mode.remove(UnixPermissions::USER_EXECUTE),
        };
        match self.group.read {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::GROUP_READ),
            Permission::Unset => mode.remove(UnixPermissions::GROUP_READ),
        };
        match self.group.write {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::GROUP_WRITE),
            Permission::Unset => mode.remove(UnixPermissions::GROUP_WRITE),
        };
        match self.group.execute {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::GROUP_EXECUTE),
            Permission::Unset => mode.remove(UnixPermissions::GROUP_EXECUTE),
        };
        match self.other.read {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::OTHER_READ),
            Permission::Unset => mode.remove(UnixPermissions::OTHER_READ),
        };
        match self.other.write {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::OTHER_WRITE),
            Permission::Unset => mode.remove(UnixPermissions::OTHER_WRITE),
        };
        match self.other.execute {
            Permission::Unchanged => return Err(crate::Error::InvalidPermission),
            Permission::Set => mode.insert(UnixPermissions::OTHER_EXECUTE),
            Permission::Unset => mode.remove(UnixPermissions::OTHER_EXECUTE),
        };

        Ok(mode.bits())
    }

    /// Return a new permission with the other permission applied.
    ///
    /// If the other permission is [Permission::Unchanged], then the original
    /// permission will be used instead.
    pub fn overwrite(&self, other: &Self) -> Self {
        Self {
            user: self.user.overwrite(&other.user),
            group: self.group.overwrite(&other.group),
            other: self.other.overwrite(&other.other),
        }
    }
}

enum FilesystemAction {
    Copy(PathBuf, PathBuf),
    Move(PathBuf, PathBuf),
    RollbackMove(PathBuf, PathBuf),
    SoftLink(PathBuf, PathBuf),
    HardLink(PathBuf, PathBuf),
    CreateDirectory(PathBuf),
    Delete(PathBuf),
    ChangeOwnerPermissions {
        path: PathBuf,
        user: Option<String>,
        group: Option<String>,
        permissions: Option<Permissions>,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum FilesystemActionType {
    Invalid,
    ChangeOwnerPermissions,
    CreateDirectory,
    CopyMoveHardLink,
    SoftLink,
    Delete,
}

/// The FilesystemTransaction type handles filesystem operations.
///
/// This includes copying paths, moving paths, hard linking paths, soft linking
/// paths, deleting paths, creating directories, and changing permissions and
/// ownership.
///
/// All of the actions except deleting can be rolled back. The permission and
/// ownership metadata will be rolled back to at the point of the transaction
/// commit.
///
/// Actions that can be grouped together will be run in parallel. For example,
/// copying, moving, or hard linking multiple paths will be done in parallel.
/// However, this also depends on the order of actions sent to the transaction.
/// So, if you copy a path, delete a path, then copy another path, then all of
/// the actions will be done one after another. However, if you copy a path,
/// copy another path, then delete a path, then the copy actions will be done in
/// parallel before the delete action.
pub struct FilesystemTransaction {
    commit_actions: Vec<FilesystemAction>,
    rollback_actions: Vec<FilesystemAction>,
    root_dir: cap_std::fs::Dir,
}

impl FilesystemTransaction {
    /// Create a new filesystem transaction.
    ///
    /// The root directory is used to control what happens in the directory. For
    /// example, if a copy goes from an arbitrary path to another arbitrary path
    /// that is not a subpath of the root, then the copy will fail. Each of the
    /// actions will state which argument must be relative to the root or not.
    pub async fn new<P: AsRef<Path>>(root_dir: P) -> Result<Self, crate::Error> {
        let auth = cap_std::ambient_authority();
        let async_root_dir_path = root_dir.as_ref().to_path_buf();
        let root_dir = tokio::task::spawn_blocking(move || {
            cap_std::fs::Dir::open_ambient_dir(async_root_dir_path, auth)
        })
        .await??;

        Ok(Self {
            commit_actions: Vec::new(),
            rollback_actions: Vec::new(),
            root_dir,
        })
    }

    /// Copy a path to the root folder.
    ///
    /// Rolling back this action will delete the target path.
    pub fn copy_path<P: AsRef<Path>>(&mut self, source_path: P, target_path: P) {
        self.commit_actions.push(FilesystemAction::Copy(
            source_path.as_ref().to_path_buf(),
            target_path.as_ref().to_path_buf(),
        ));
    }

    /// Move a path to the root folder.
    ///
    /// Rolling back this action will move the path back to its original
    /// location.
    pub fn move_path<P: AsRef<Path>>(&mut self, source_path: P, target_path: P) {
        self.commit_actions.push(FilesystemAction::Move(
            source_path.as_ref().to_path_buf(),
            target_path.as_ref().to_path_buf(),
        ));
    }

    /// Hard link a path to the root folder.
    ///
    /// Rolling back this action will delete the target path.
    ///
    /// The source and target paths are relative to the root because this
    /// assumes that each root directory is a self-contained and immutable
    /// package. Any file that is outside of the root directory is assumed to be
    /// mutable.
    pub fn hard_link_path<P: AsRef<Path>>(&mut self, source_path: P, target_path: P) {
        self.commit_actions.push(FilesystemAction::HardLink(
            source_path.as_ref().to_path_buf(),
            target_path.as_ref().to_path_buf(),
        ));
    }

    /// Soft link a path to the root folder.
    ///
    /// Rolling back this action will delete the target path.
    ///
    /// The source and target paths are relative to the root because this
    /// assumes that each root directory is a self-contained and immutable
    /// package. Any path that is outside of the root directory is assumed to be
    /// mutable.
    pub fn soft_link_path<P: AsRef<Path>>(&mut self, source_path: P, target_path: P) {
        self.commit_actions.push(FilesystemAction::SoftLink(
            source_path.as_ref().to_path_buf(),
            target_path.as_ref().to_path_buf(),
        ));
    }

    /// Change the owner and permissions of a path.
    ///
    /// Rolling back this action will change the owner and permissions back to
    /// what they were before the transaction was committed.
    pub fn change_owner_permissions<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        user: Option<S>,
        group: Option<S>,
        permissions: Option<Permissions>,
    ) {
        self.commit_actions
            .push(FilesystemAction::ChangeOwnerPermissions {
                path: path.as_ref().to_path_buf(),
                user: user.map(|u| u.as_ref().to_string()),
                group: group.map(|g| g.as_ref().to_string()),
                permissions,
            });
    }

    /// Create a directory.
    ///
    /// Rolling back this action will delete the directory.
    pub fn create_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.commit_actions.push(FilesystemAction::CreateDirectory(
            path.as_ref().to_path_buf(),
        ));
    }

    /// Delete a path.
    ///
    /// This action cannot be rolled back.
    pub fn delete_path<P: AsRef<Path>>(&mut self, path: P) {
        self.commit_actions
            .push(FilesystemAction::Delete(path.as_ref().to_path_buf()));
    }

    async fn run_actions(
        root_dir: &cap_std::fs::Dir,
        buf: &mut Vec<FilesystemAction>,
        tasks: &mut tokio::task::JoinSet<Result<(), crate::Error>>,
    ) -> Result<(), crate::Error> {
        // TODO: Actions should be run in tasks
        for action in buf.iter() {
            match action {
                FilesystemAction::Copy(source_path, target_path) => {
                    Self::run_copy_action(root_dir, source_path, target_path).await?;
                }
                FilesystemAction::Move(source_path, target_path) => {
                    Self::run_move_action(root_dir, source_path, target_path).await?;
                }
                FilesystemAction::RollbackMove(source_path, target_path) => {
                    Self::run_rollback_move_action(root_dir, source_path, target_path).await?;
                }
                FilesystemAction::HardLink(source_path, target_path) => {
                    Self::run_hard_link_action(root_dir, source_path, target_path).await?;
                }
                FilesystemAction::SoftLink(source_path, target_path) => {
                    Self::run_soft_link_action(root_dir, source_path, target_path).await?;
                }
                FilesystemAction::CreateDirectory(path) => {
                    Self::run_create_directory_action(root_dir, path).await?;
                }
                FilesystemAction::Delete(path) => {
                    Self::run_delete_action(root_dir, path).await?;
                }
                FilesystemAction::ChangeOwnerPermissions {
                    path,
                    user,
                    group,
                    permissions,
                } => {
                    Self::run_change_owner_permissions_action(
                        root_dir,
                        path,
                        user,
                        group,
                        permissions,
                    )
                    .await?;
                }
            }
        }

        while let Some(task) = tasks.join_next().await {
            task??
        }

        buf.clear();

        Ok(())
    }

    async fn run_copy_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let metadata = tokio::fs::metadata(&source_path).await?;
        if metadata.is_file() {
            copy_file(root_dir, source_path, target_path).await
        } else if metadata.is_dir() {
            copy_directory(root_dir, source_path, target_path).await
        } else {
            Err(crate::Error::InvalidMetadata(source_path.to_path_buf()))
        }
    }

    async fn run_move_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let source_path = source_path.to_path_buf();
        let target_path = target_path.to_path_buf();
        let root_dir = root_dir.try_clone()?;

        tokio::task::spawn_blocking(move || {
            let source_dir = match source_path.parent() {
                Some(p) => p,
                None => return Err(crate::Error::SourcePathInvalid(source_path)),
            };
            let source_name = match source_path.file_name() {
                Some(n) => n,
                None => return Err(crate::Error::SourcePathInvalid(source_path)),
            };

            let source_dir =
                cap_std::fs::Dir::open_ambient_dir(source_dir, cap_std::ambient_authority())?;

            Ok(source_dir.rename(source_name, &root_dir, target_path)?)
        })
        .await?
    }

    async fn run_rollback_move_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let source_path = source_path.to_path_buf();
        let target_path = target_path.to_path_buf();
        let root_dir = root_dir.try_clone()?;

        tokio::task::spawn_blocking(move || {
            let source_dir = match source_path.parent() {
                Some(p) => p,
                None => return Err(crate::Error::SourcePathInvalid(source_path)),
            };
            let source_name = match source_path.file_name() {
                Some(n) => n,
                None => return Err(crate::Error::SourcePathInvalid(source_path)),
            };
            let target_name = match target_path.file_name() {
                Some(n) => n,
                None => return Err(crate::Error::TargetPathInvalid(target_path)),
            };

            let source_dir =
                cap_std::fs::Dir::open_ambient_dir(source_dir, cap_std::ambient_authority())?;

            Ok(root_dir.rename(target_name, &source_dir, source_name)?)
        })
        .await?
    }

    async fn run_hard_link_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let source_path = source_path.to_path_buf();
        let target_path = target_path.to_path_buf();
        let root_dir = root_dir.try_clone()?;
        tokio::task::spawn_blocking(move || {
            let source_name = match source_path.file_name() {
                Some(n) => n,
                None => return Err(crate::Error::SourcePathInvalid(source_path)),
            };

            match root_dir.hard_link(source_name, &root_dir, target_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(crate::Error::from(err)),
            }
        })
        .await?
    }

    #[cfg(unix)]
    async fn run_soft_link_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let async_source_path = source_path.to_path_buf();
        let async_target_path = target_path.to_path_buf();
        let root_dir = root_dir.try_clone()?;
        tokio::task::spawn_blocking(move || {
            match root_dir.symlink(async_source_path, async_target_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(crate::Error::from(err)),
            }
        })
        .await?
    }

    #[cfg(windows)]
    async fn run_soft_link_action(
        root_dir: &cap_std::fs::Dir,
        source_path: &Path,
        target_path: &Path,
    ) -> Result<(), crate::Error> {
        let source_path = source_path.to_path_buf();
        let target_path = target_path.to_path_buf();
        let root_dir = root_dir.try_clone()?;
        tokio::task::spawn_blocking(move || {
            let metadata = source_path.metadata()?;

            if metadata.is_file() {
                match root_dir.symlink_file(source_path, target_path) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(crate::Error::from(err)),
                }
            } else if metadata.is_dir() {
                match root_dir.symlink_dir(source_path, target_path) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(crate::Error::from(err)),
                }
            } else {
                Err(crate::Error::InvalidMetadata(source_path))
            }
        })
        .await?
    }

    async fn run_create_directory_action(
        root_dir: &cap_std::fs::Dir,
        path: &Path,
    ) -> Result<(), crate::Error> {
        let path = path.to_path_buf();
        let root_dir = root_dir.try_clone()?;
        tokio::task::spawn_blocking(move || match root_dir.create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(crate::Error::from(err)),
        })
        .await?
    }

    async fn run_delete_action(
        root_dir: &cap_std::fs::Dir,
        path: &Path,
    ) -> Result<(), crate::Error> {
        let path = path.to_path_buf();
        let root_dir = root_dir.try_clone()?;
        tokio::task::spawn_blocking(move || {
            let metadata = root_dir.metadata(&path)?;

            if metadata.is_file() {
                Ok(root_dir.remove_file(&path)?)
            } else if metadata.is_dir() {
                Ok(root_dir.remove_dir_all(&path)?)
            } else {
                Err(crate::Error::InvalidMetadata(path))
            }
        })
        .await?
    }

    #[cfg(unix)]
    async fn run_change_owner_permissions_action(
        root_dir: &cap_std::fs::Dir,
        path: &Path,
        user: &Option<String>,
        group: &Option<String>,
        permissions: &Option<Permissions>,
    ) -> Result<(), crate::Error> {
        let root_dir = root_dir.try_clone()?;
        let path = path.to_path_buf();
        let user = user.to_owned();
        let group = group.to_owned();
        let permissions = permissions.to_owned();
        tokio::task::spawn_blocking(move || {
            use std::os::unix::fs::PermissionsExt;

            if let Some(permissions) = permissions {
                let mut current_permissions = root_dir.metadata(&path)?.permissions();
                let current_mode = current_permissions.mode();
                let new_permissions = Permissions::from_mode(current_mode).overwrite(&permissions);
                current_permissions.set_mode(new_permissions.as_mode()?);
                root_dir.set_permissions(&path, current_permissions)?;
            }

            if user.is_none() && group.is_none() {
                Ok(())
            } else {
                let uid = match user {
                    Some(user) => match nix::unistd::User::from_name(&user)? {
                        // TODO: May need to swap rustix for nix or visa versa.
                        Some(u) => Some(unsafe { rustix::process::Uid::from_raw(u.uid.as_raw()) }),
                        None => None,
                    },
                    None => None,
                };
                let gid = match group {
                    Some(group) => match nix::unistd::Group::from_name(&group)? {
                        // TODO: May need to swap rustix for nix or visa versa.
                        Some(g) => Some(unsafe { rustix::process::Gid::from_raw(g.gid.as_raw()) }),
                        None => None,
                    },
                    None => None,
                };

                rustix::fs::chownat(
                    root_dir,
                    path,
                    uid,
                    gid,
                    rustix::fs::AtFlags::SYMLINK_NOFOLLOW,
                )?;

                Ok(())
            }
        })
        .await?
    }

    #[cfg(windows)]
    async fn run_change_owner_permissions_action(
        root_dir: &cap_std::fs::Dir,
        path: &Path,
        user: &Option<String>,
        group: &Option<String>,
        permissions: &Option<Permissions>,
    ) -> Result<(), crate::Error> {
        todo!()
    }
}

#[async_trait::async_trait]
impl crate::transactions::Transaction for FilesystemTransaction {
    async fn commit(&mut self) -> Result<(), crate::Error> {
        self.rollback_actions.clear();

        let mut last_action_type = FilesystemActionType::Invalid;
        let mut tasks: tokio::task::JoinSet<Result<(), crate::Error>> = tokio::task::JoinSet::new();
        let mut buf: Vec<FilesystemAction> = Vec::new();

        for action in &mut self.commit_actions {
            match action {
                FilesystemAction::Copy(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::Copy(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                    self.rollback_actions
                        .push(FilesystemAction::Delete(target_path.to_path_buf()));
                }
                FilesystemAction::Move(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::Move(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                    self.rollback_actions.push(FilesystemAction::RollbackMove(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::RollbackMove(_source_path, _target_path) => {
                    panic!("Should not be able to do a rollback move in commit.");
                }
                FilesystemAction::HardLink(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::HardLink(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                    self.rollback_actions
                        .push(FilesystemAction::Delete(target_path.to_path_buf()));
                }
                FilesystemAction::SoftLink(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::SoftLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::SoftLink;
                    }

                    buf.push(FilesystemAction::SoftLink(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                    self.rollback_actions
                        .push(FilesystemAction::Delete(target_path.to_path_buf()));
                }
                FilesystemAction::CreateDirectory(path) => {
                    if last_action_type != FilesystemActionType::CreateDirectory {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CreateDirectory;
                    }

                    buf.push(FilesystemAction::CreateDirectory(path.to_path_buf()));
                    self.rollback_actions
                        .push(FilesystemAction::Delete(path.to_path_buf()));
                }
                FilesystemAction::Delete(path) => {
                    if last_action_type != FilesystemActionType::Delete {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::Delete;
                    }

                    buf.push(FilesystemAction::Delete(path.to_path_buf()));
                }
                FilesystemAction::ChangeOwnerPermissions {
                    path,
                    user,
                    group,
                    permissions,
                } => {
                    if last_action_type != FilesystemActionType::ChangeOwnerPermissions {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::ChangeOwnerPermissions;
                    }

                    buf.push(FilesystemAction::ChangeOwnerPermissions {
                        path: path.to_path_buf(),
                        user: user.to_owned(),
                        group: group.to_owned(),
                        permissions: permissions.to_owned(),
                    });
                    let (current_user, current_group, current_permissions) =
                        get_current_owner_and_permissions(&self.root_dir, path).await?;
                    self.rollback_actions
                        .push(FilesystemAction::ChangeOwnerPermissions {
                            path: path.to_path_buf(),
                            user: current_user,
                            group: current_group,
                            permissions: Some(current_permissions),
                        });
                }
            }
        }

        if !buf.is_empty() {
            Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
        }

        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), crate::Error> {
        let mut last_action_type = FilesystemActionType::Invalid;
        let mut tasks: tokio::task::JoinSet<Result<(), crate::Error>> = tokio::task::JoinSet::new();
        let mut buf: Vec<FilesystemAction> = Vec::new();

        for action in &self.rollback_actions {
            match action {
                FilesystemAction::Copy(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::Copy(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::Move(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::Move(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::RollbackMove(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::RollbackMove(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::HardLink(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::CopyMoveHardLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CopyMoveHardLink;
                    }

                    buf.push(FilesystemAction::HardLink(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::SoftLink(source_path, target_path) => {
                    if last_action_type != FilesystemActionType::SoftLink {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::SoftLink;
                    }

                    buf.push(FilesystemAction::SoftLink(
                        source_path.to_path_buf(),
                        target_path.to_path_buf(),
                    ));
                }
                FilesystemAction::CreateDirectory(path) => {
                    if last_action_type != FilesystemActionType::CreateDirectory {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::CreateDirectory;
                    }

                    buf.push(FilesystemAction::CreateDirectory(path.to_path_buf()));
                }
                FilesystemAction::Delete(path) => {
                    if last_action_type != FilesystemActionType::Delete {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::Delete;
                    }

                    buf.push(FilesystemAction::Delete(path.to_path_buf()));
                }
                FilesystemAction::ChangeOwnerPermissions {
                    path,
                    user,
                    group,
                    permissions,
                } => {
                    if last_action_type != FilesystemActionType::ChangeOwnerPermissions {
                        Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
                        last_action_type = FilesystemActionType::ChangeOwnerPermissions;
                    }

                    buf.push(FilesystemAction::ChangeOwnerPermissions {
                        path: path.to_path_buf(),
                        user: user.to_owned(),
                        group: group.to_owned(),
                        permissions: permissions.to_owned(),
                    });
                }
            }
        }

        if !buf.is_empty() {
            Self::run_actions(&self.root_dir, &mut buf, &mut tasks).await?;
        }

        Ok(())
    }
}

async fn copy_file(
    root_dir: &cap_std::fs::Dir,
    source_path: &Path,
    target_path: &Path,
) -> Result<(), crate::Error> {
    let root_dir = root_dir.try_clone()?;
    let target_path = target_path.to_path_buf();

    let mut f_in = tokio::fs::File::open(&source_path).await?;
    let f_out = tokio::task::spawn_blocking(move || root_dir.create(target_path)).await??;
    let mut f_out = tokio::fs::File::from(f_out.into_std());

    tokio::io::copy(&mut f_in, &mut f_out).await?;

    Ok(())
}

#[async_recursion::async_recursion]
async fn copy_directory(
    root_dir: &cap_std::fs::Dir,
    source_path: &Path,
    target_path: &Path,
) -> Result<(), crate::Error> {
    let mut source_dir_iter = tokio::fs::read_dir(source_path).await?;
    let mut tasks = tokio::task::JoinSet::new();

    let async_root_dir = root_dir.try_clone()?;
    let async_target_path = target_path.to_path_buf();
    tokio::task::spawn_blocking(move || async_root_dir.create_dir(async_target_path)).await??;

    while let Some(source_dir_item) = source_dir_iter.next_entry().await? {
        let root_dir = root_dir.try_clone()?;
        let source_path = source_path.to_path_buf();
        let target_path = target_path.to_path_buf();

        tasks.spawn(async move {
            let metadata = source_dir_item.metadata().await?;
            let source_child_path = source_dir_item.path();
            let mut target_child_path = target_path.to_path_buf();
            target_child_path.push(source_dir_item.file_name());

            if metadata.is_file() {
                copy_file(&root_dir, &source_child_path, &target_child_path).await?;
            } else if metadata.is_dir() {
                copy_directory(&root_dir, &source_child_path, &target_child_path).await?;
            } else {
                return Err(crate::Error::InvalidMetadata(source_path.to_path_buf()));
            }

            Ok(())
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(r) => r?,
            Err(err) => {
                return Err(crate::Error::from(err));
            }
        };
    }

    Ok(())
}

#[cfg(unix)]
async fn get_current_owner_and_permissions(
    root_dir: &cap_std::fs::Dir,
    path: &Path,
) -> Result<(Option<String>, Option<String>, Permissions), crate::Error> {
    let root_dir = root_dir.try_clone()?;
    let path = path.to_path_buf();

    tokio::task::spawn_blocking(move || {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let metadata = root_dir.metadata(path)?;
        let mode = metadata.permissions().mode();
        let uid = metadata.uid();
        let gid = metadata.gid();

        let user = match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid))? {
            Some(u) => Some(u.name),
            None => None,
        };
        let group = match nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(gid))? {
            Some(g) => Some(g.name),
            None => None,
        };
        let permissions = Permissions::from_mode(mode);

        Ok((user, group, permissions))
    })
    .await?
}

#[cfg(windows)]
async fn get_current_owner_and_permissions(
    root_dir: &cap_std::fs::Dir,
    path: &Path,
) -> Result<(Option<String>, Option<String>, Permissions), crate::Error> {
    todo!()
}
