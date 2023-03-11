use std::{
    io::{Read, Write},
    path::PathBuf,
};

use publish::transactions::Transaction;

#[tokio::test]
async fn test_copy_file_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_source_dir.path();
    let target_dir = tmp_target_dir.path();
    let mut source_file = PathBuf::from(source_dir);
    source_file.push("test");
    let mut target_file = PathBuf::from(target_dir);
    target_file.push("test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    transaction.copy_path(&source_file, &PathBuf::from("test"));
    transaction.commit().await.unwrap();

    assert!(source_file.is_file());
    assert!(target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[tokio::test]
async fn test_copy_files_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_source_dir.path();
    let target_dir = tmp_target_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.copy_path(&source_file, &PathBuf::from(file_name));
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.is_file(), "{:?}", target_file);
    }
}

#[tokio::test]
async fn test_copy_file_failure_invalid_target_dir() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_source_dir.path());
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_target_dir.path());
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.copy_path(&source_file, &target_file);
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_copy_dir_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_dir = PathBuf::from(tmp_source_dir.path());
    source_dir.push("test");
    let mut target_dir = PathBuf::from(tmp_target_dir.path());
    target_dir.push("test");
    let mut source_file = PathBuf::from(&source_dir);
    source_file.push("test");
    let mut target_file = PathBuf::from(&target_dir);
    target_file.push("test");

    std::fs::create_dir_all(&source_dir).unwrap();
    assert!(source_dir.is_dir());

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_target_dir.path())
        .await
        .unwrap();

    transaction.copy_path(&source_dir, &PathBuf::from("test"));
    transaction.commit().await.unwrap();

    assert!(source_file.is_file(), "{:?}", source_file);
    assert!(target_file.is_file(), "{:?}", target_file);

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(source_file.parent().unwrap().is_dir());
    assert!(!target_file.parent().unwrap().exists());
}

#[tokio::test]
async fn test_copy_dirs_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    // let source_dir = tmp_source_dir.path();
    // let target_dir = tmp_target_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_target_dir.path())
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.copy_path(source_file.parent().unwrap(), &PathBuf::from(file_name));
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_target_dir.path());
        target_file.push(file_name);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_target_dir.path());
        target_file.push(file_name);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
        assert!(source_file.parent().unwrap().is_dir());
        assert!(!target_file.parent().unwrap().exists());
    }
}

#[tokio::test]
async fn test_copy_dir_failure_invalid_target_dir() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_source_dir.path());
    source_file.push("test");
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_target_dir.path());
    target_file.push("test");
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("test");
    other_target_file.push("test");

    std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.copy_path(source_file.parent().unwrap(), target_file.parent().unwrap());
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_move_file_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_source_dir.path();
    let target_dir = tmp_target_dir.path();
    let mut source_file = PathBuf::from(source_dir);
    source_file.push("test");
    let mut target_file = PathBuf::from(target_dir);
    target_file.push("test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    transaction.move_path(&source_file, &PathBuf::from("test"));
    transaction.commit().await.unwrap();

    assert!(!source_file.is_file());
    assert!(target_file.is_file());

    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(t_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[tokio::test]
async fn test_move_files_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_source_dir.path();
    let target_dir = tmp_target_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.move_path(&source_file, &PathBuf::from(file_name));
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(file_name);
        assert!(!source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(t_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
    }
}

#[tokio::test]
async fn test_move_file_failure_invalid_target_dir() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_source_dir.path());
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_target_dir.path());
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.move_path(&source_file, &target_file);
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_move_dir_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_dir = PathBuf::from(tmp_source_dir.path());
    source_dir.push("test");
    let mut target_dir = PathBuf::from(tmp_target_dir.path());
    target_dir.push("test");
    let mut source_file = PathBuf::from(&source_dir);
    source_file.push("test");
    let mut target_file = PathBuf::from(&target_dir);
    target_file.push("test");

    std::fs::create_dir_all(&source_dir).unwrap();
    assert!(source_dir.is_dir());

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_target_dir.path())
        .await
        .unwrap();

    transaction.move_path(&source_dir, &PathBuf::from("test"));
    transaction.commit().await.unwrap();

    assert!(!source_file.is_file(), "{:?}", source_file);
    assert!(target_file.is_file(), "{:?}", target_file);

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(source_file.parent().unwrap().is_dir());
    assert!(!target_file.parent().unwrap().exists());
}

#[tokio::test]
async fn test_move_dirs_success() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();

    // let source_dir = tmp_source_dir.path();
    // let target_dir = tmp_target_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_target_dir.path())
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.move_path(source_file.parent().unwrap(), &PathBuf::from(file_name));
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_target_dir.path());
        target_file.push(file_name);
        target_file.push(file_name);
        assert!(!source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_source_dir.path());
        source_file.push(file_name);
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_target_dir.path());
        target_file.push(file_name);
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
        assert!(source_file.parent().unwrap().is_dir());
        assert!(!target_file.parent().unwrap().exists());
    }
}

#[tokio::test]
async fn test_move_dir_failure_invalid_target_dir() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_source_dir.path());
    source_file.push("test");
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_target_dir.path());
    target_file.push("test");
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("test");
    other_target_file.push("test");

    std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.move_path(source_file.parent().unwrap(), target_file.parent().unwrap());
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());
}

#[cfg(unix)]
#[tokio::test]
async fn test_unix_hard_link_file_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let mut source_file = PathBuf::from(source_dir);
    source_file.push("source");
    let mut target_file = PathBuf::from(target_dir);
    target_file.push("target");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    transaction.hard_link_path(&PathBuf::from("source"), &PathBuf::from("target"));
    transaction.commit().await.unwrap();

    assert!(source_file.is_file());
    assert!(target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    use std::os::unix::fs::MetadataExt;
    let source_metadata = std::fs::metadata(&source_file).unwrap();
    let target_metadata = std::fs::metadata(&target_file).unwrap();

    assert_eq!(source_metadata.ino(), target_metadata.ino());

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[cfg(windows)]
#[tokio::test]
async fn test_windows_hard_link_file_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let mut source_file = PathBuf::from(source_dir);
    source_file.push("source");
    let mut target_file = PathBuf::from(target_dir);
    target_file.push("target");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    transaction.hard_link_path(&PathBuf::from("source"), &PathBuf::from("target"));
    transaction.commit().await.unwrap();

    assert!(source_file.is_file());
    assert!(target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[cfg(unix)]
#[tokio::test]
async fn test_unix_hard_link_files_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.hard_link_path(
            format!("source_{}", file_name),
            format!("target_{}", file_name),
        );
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);

        use std::os::unix::fs::MetadataExt;
        let source_metadata = std::fs::metadata(&source_file).unwrap();
        let target_metadata = std::fs::metadata(&target_file).unwrap();

        assert_eq!(source_metadata.ino(), target_metadata.ino());
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
    }
}

#[cfg(windows)]
#[tokio::test]
async fn test_windows_hard_link_files_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.hard_link_path(
            format!("source_{}", file_name),
            format!("target_{}", file_name),
        );
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
    }
}

#[tokio::test]
async fn test_hard_link_file_failure_invalid_target_dir() {
    let tmp_source_dir = tempfile::TempDir::new().unwrap();
    let tmp_target_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_source_dir.path());
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_target_dir.path());
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.hard_link_path(&source_file, &target_file);
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_soft_link_file_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let mut source_file = PathBuf::from(source_dir);
    source_file.push("source");
    let mut target_file = PathBuf::from(target_dir);
    target_file.push("target");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    transaction.soft_link_path(&PathBuf::from("source"), &PathBuf::from("target"));
    transaction.commit().await.unwrap();

    assert!(source_file.is_file());
    assert!(target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    assert!(target_file.is_symlink());

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[tokio::test]
async fn test_soft_link_files_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let source_dir = tmp_root_dir.path();
    let target_dir = tmp_root_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(target_dir)
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.soft_link_path(
            format!("source_{}", file_name),
            format!("target_{}", file_name),
        );
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);
        assert!(target_file.is_symlink());
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(source_dir);
        source_file.push(format!("source_{}", file_name));
        let mut target_file = PathBuf::from(target_dir);
        target_file.push(format!("target_{}", file_name));
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
    }
}

#[tokio::test]
async fn test_soft_link_file_failure_invalid_target_dir() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_root_dir.path());
    source_file.push("source_test");
    let mut target_file = PathBuf::from(tmp_root_dir.path());
    target_file.push("target_test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("other_target_test");

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.soft_link_path(&source_file, &target_file);
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_soft_link_dir_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let mut source_dir = PathBuf::from(tmp_root_dir.path());
    source_dir.push("source_test");
    let mut target_dir = PathBuf::from(tmp_root_dir.path());
    target_dir.push("target_test");
    let mut source_file = PathBuf::from(&source_dir);
    source_file.push("test");
    let mut target_file = PathBuf::from(&target_dir);
    target_file.push("test");

    std::fs::create_dir_all(&source_dir).unwrap();
    assert!(source_dir.is_dir());

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    transaction.soft_link_path("source_test", "target_test");
    transaction.commit().await.unwrap();

    assert!(source_file.is_file(), "{:?}", source_file);
    assert!(target_file.is_file(), "{:?}", target_file);

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();
    let mut t_f = std::fs::File::open(&target_file).unwrap();
    let mut t_v = Vec::new();
    t_f.read_to_end(&mut t_v).unwrap();

    assert_eq!(s_v, t_v);

    assert!(target_file.parent().unwrap().is_symlink());

    transaction.rollback().await.unwrap();
    assert!(source_file.is_file());
    assert!(!target_file.exists());
}

#[tokio::test]
async fn test_soft_link_dirs_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    // let source_dir = tmp_root_dir.path();
    // let target_dir = tmp_root_dir.path();
    let file_names = ["test1", "test2", "test3"];

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_root_dir.path());
        source_file.push(format!("source_{}", file_name));
        source_file.push(file_name);
        std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

        let mut f = std::fs::File::create(&source_file).unwrap();
        f.write_all(b"test").unwrap();

        transaction.soft_link_path(
            format!("source_{}", file_name),
            format!("target_{}", file_name),
        );
    }

    transaction.commit().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_root_dir.path());
        source_file.push(format!("source_{}", file_name));
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_root_dir.path());
        target_file.push(format!("target_{}", file_name));
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(target_file.is_file(), "{:?}", target_file);

        let mut s_f = std::fs::File::open(&source_file).unwrap();
        let mut s_v = Vec::new();
        s_f.read_to_end(&mut s_v).unwrap();
        let mut t_f = std::fs::File::open(&target_file).unwrap();
        let mut t_v = Vec::new();
        t_f.read_to_end(&mut t_v).unwrap();

        assert_eq!(s_v, t_v);

        assert!(target_file.parent().unwrap().is_symlink());
    }

    transaction.rollback().await.unwrap();

    for file_name in &file_names {
        let mut source_file = PathBuf::from(tmp_root_dir.path());
        source_file.push(format!("source_{}", file_name));
        source_file.push(file_name);
        let mut target_file = PathBuf::from(tmp_root_dir.path());
        target_file.push(format!("target_{}", file_name));
        target_file.push(file_name);
        assert!(source_file.is_file(), "{:?}", source_file);
        assert!(!target_file.exists(), "{:?}", target_file);
    }
}

#[tokio::test]
async fn test_soft_link_dir_failure_invalid_target_dir() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();
    let tmp_other_target_dir = tempfile::TempDir::new().unwrap();

    let mut source_file = PathBuf::from(tmp_root_dir.path());
    source_file.push("source_test");
    source_file.push("test");
    let mut target_file = PathBuf::from(tmp_root_dir.path());
    target_file.push("target_test");
    target_file.push("test");
    let mut other_target_file = PathBuf::from(tmp_other_target_dir.path());
    other_target_file.push("other_target_test");
    other_target_file.push("test");

    std::fs::create_dir_all(source_file.parent().unwrap()).unwrap();

    let mut f = std::fs::File::create(&source_file).unwrap();
    f.write_all(b"test").unwrap();

    let mut transaction =
        publish::transactions::FilesystemTransaction::new(tmp_other_target_dir.path())
            .await
            .unwrap();

    transaction.soft_link_path(source_file.parent().unwrap(), target_file.parent().unwrap());
    assert!(transaction.commit().await.is_err());

    assert!(source_file.is_file());
    assert!(!target_file.exists());
    assert!(!other_target_file.is_file());

    let mut s_f = std::fs::File::open(&source_file).unwrap();
    let mut s_v = Vec::new();
    s_f.read_to_end(&mut s_v).unwrap();

    assert_eq!(s_v, b"test".iter().map(|v| *v).collect::<Vec<u8>>());
}

#[tokio::test]
async fn test_create_dir_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let mut new_dir = PathBuf::from(tmp_root_dir.path());
    new_dir.push("test");

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    transaction.create_directory("test");
    transaction.commit().await.unwrap();

    assert!(new_dir.is_dir(), "{:?}", new_dir);

    transaction.rollback().await.unwrap();
    assert!(!new_dir.exists(), "{:?}", new_dir);
}

#[tokio::test]
async fn test_delete_file_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let mut test_file = PathBuf::from(tmp_root_dir.path());
    test_file.push("test");

    let mut f = std::fs::File::create(&test_file).unwrap();
    f.write_all(b"test").unwrap();
    assert!(test_file.exists(), "{:?}", test_file);

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    transaction.delete_path("test");
    transaction.commit().await.unwrap();

    assert!(!test_file.exists(), "{:?}", test_file);

    transaction.rollback().await.unwrap();
    assert!(!test_file.exists(), "{:?}", test_file);
}

#[tokio::test]
async fn test_delete_dir_success() {
    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let mut test_file = PathBuf::from(tmp_root_dir.path());
    test_file.push("test");
    test_file.push("test");

    std::fs::create_dir_all(test_file.parent().unwrap()).unwrap();
    let mut f = std::fs::File::create(&test_file).unwrap();
    f.write_all(b"test").unwrap();
    assert!(test_file.exists(), "{:?}", test_file);

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    transaction.delete_path("test");
    transaction.commit().await.unwrap();

    assert!(!test_file.exists(), "{:?}", test_file);
    assert!(!test_file.parent().unwrap().exists(), "{:?}", test_file);

    transaction.rollback().await.unwrap();
    assert!(!test_file.exists(), "{:?}", test_file);
}

#[cfg(unix)]
#[tokio::test]
async fn test_change_file_owner_permissions() {
    use std::os::unix::fs::MetadataExt;

    let tmp_root_dir = tempfile::TempDir::new().unwrap();

    let mut test_file = PathBuf::from(tmp_root_dir.path());
    test_file.push("test");

    let mut f = std::fs::File::create(&test_file).unwrap();
    f.write_all(b"test").unwrap();
    assert!(test_file.exists(), "{:?}", test_file);
    let initial_metadata = std::fs::metadata(&test_file).unwrap();

    let mut transaction = publish::transactions::FilesystemTransaction::new(tmp_root_dir.path())
        .await
        .unwrap();

    // Unchanged
    transaction.change_owner_permissions::<&str, &str>("test", None, None, None);
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Set user and group to current user and group
    let user = nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(initial_metadata.uid()))
        .unwrap()
        .unwrap()
        .name;
    let group = nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(initial_metadata.gid()))
        .unwrap()
        .unwrap()
        .name;
    transaction.change_owner_permissions::<&str, &str>("test", Some(&user), Some(&group), None);
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Unchanged: Permissions set to default "unchanged"
    let permissions = publish::transactions::Permissions::default();
    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only user read
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Set;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100400);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only user write
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Set;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100200);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only user execute
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Set;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100100);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only group read
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Set;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100040);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only group write
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Set;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100020);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only group execute
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Set;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100010);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only other read
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Set;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100004);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only other write
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Set;
    permissions.other.execute = publish::transactions::Permission::Unset;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100002);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // Only other execute
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Unset;
    permissions.user.write = publish::transactions::Permission::Unset;
    permissions.user.execute = publish::transactions::Permission::Unset;
    permissions.group.read = publish::transactions::Permission::Unset;
    permissions.group.write = publish::transactions::Permission::Unset;
    permissions.group.execute = publish::transactions::Permission::Unset;
    permissions.other.read = publish::transactions::Permission::Unset;
    permissions.other.write = publish::transactions::Permission::Unset;
    permissions.other.execute = publish::transactions::Permission::Set;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100001);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());

    // All
    let mut permissions = publish::transactions::Permissions::default();
    permissions.user.read = publish::transactions::Permission::Set;
    permissions.user.write = publish::transactions::Permission::Set;
    permissions.user.execute = publish::transactions::Permission::Set;
    permissions.group.read = publish::transactions::Permission::Set;
    permissions.group.write = publish::transactions::Permission::Set;
    permissions.group.execute = publish::transactions::Permission::Set;
    permissions.other.read = publish::transactions::Permission::Set;
    permissions.other.write = publish::transactions::Permission::Set;
    permissions.other.execute = publish::transactions::Permission::Set;

    transaction.change_owner_permissions::<&str, &str>("test", None, None, Some(permissions));
    transaction.commit().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), 0o100777);

    transaction.rollback().await.unwrap();
    let result_metadata = std::fs::metadata(&test_file).unwrap();

    assert_eq!(result_metadata.uid(), initial_metadata.uid());
    assert_eq!(result_metadata.gid(), initial_metadata.gid());
    assert_eq!(result_metadata.mode(), initial_metadata.mode());
}
