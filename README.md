# Untitled Publish Framework

## Overview

This framework is designed to provide a way to publish data. This may include
saving data to the filesystem, or updating metadata in a database.

## Features

- A Rust, C, and Python 3 API
- A pre-publish, publish, post-publish step
- A transaction system for performing filesystem, database, etc actions
- Rolling back transactions if failed

## Requirements

- Make
- Rust: 1.66 or later (This is not the guaranteed minimum supported Rust
  version)
- Python: 3.7 or later

## Install

```bash
cd /to/your/project
cargo add --git https://github.com/scott-wilson/publish.git
```

## Design

### Transactions

Transactions are responsible for making publishes permanent. They could be used
for filesystem operations such as copying, moving, or hard linking. Or, they
could be used for registering a publish entity on the database.

### Publishes

Publishes are a collection of transactions and data transformers. The publishes
have the following stages:

- Pre-publish: This is used to prepare a publish. It could include registering a
  publish entity on the database, and creating a directory to publish to.
- Publish: This is the main body of work. For example, optimizing assets for
  publishes, generating caches, etc. Then, saving publishes to the publish
  directory.
- Post-publish: This is for finalizing the publish. For example, marking the
  publish entity as ready, locking the directory, etc.

### Runner

The runner will run the publish and return the final result. If any of the
publish stages fail, then it will try to roll back the transactions that have
run.
