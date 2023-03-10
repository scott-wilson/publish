"""The transactions package is responsible for creating new transactions.

Transactions are the heart of pypublish. They are used to publish files,
register a publish to the database, etc. A publish is only a collection of
transactions and transformations to the data.

There are two implemented transactions:

- :code:`FilesystemTransaction`: This transaction is used to publish files.
  This may include copying, moving, hard/soft linking, changing permissions,
  creating folders, etc.
- :code:`RootTransaction`: This transaction is used to group other
  transactions. It is unlikely that a publish will use this transaction
  directly, but could still be useful if transactions need to be done as a
  hierarchy of transactions.
"""

from ._filesystem_transaction import (
    FilesystemTransaction,
    Permission,
    Permissions,
    ScopedPermission,
)
from ._root_transaction import RootTransaction
from ._transaction import Transaction

__all__ = [
    "FilesystemTransaction",
    "Permission",
    "Permissions",
    "RootTransaction",
    "ScopedPermission",
    "Transaction",
]
