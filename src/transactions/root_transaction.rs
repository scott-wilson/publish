use std::sync::Arc;
use tokio::sync::Mutex;

type ChildTransaction = Mutex<Box<dyn crate::transactions::Transaction + Sync + Send>>;

pub struct RootTransaction {
    children: Vec<Vec<Arc<ChildTransaction>>>,
}

impl RootTransaction {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child<T: crate::transactions::Transaction + Sync + Send + 'static>(
        &mut self,
        child: T,
    ) {
        self.children
            .push(vec![Arc::new(Mutex::new(Box::new(child)))]);
    }

    pub fn add_child_parallel<T: crate::transactions::Transaction + Sync + Send + 'static>(
        &mut self,
        child: T,
    ) {
        let children = match self.children.last_mut() {
            Some(c) => c,
            None => {
                self.children.push(Vec::new());
                let index = self.children.len() - 1;
                &mut self.children[index]
            }
        };

        children.push(Arc::new(Mutex::new(Box::new(child))));
    }
}

#[async_trait::async_trait]
impl crate::transactions::Transaction for RootTransaction {
    async fn commit(&mut self) -> Result<(), crate::Error> {
        let mut tasks = tokio::task::JoinSet::new();
        let mut errors = Vec::with_capacity(self.children.len());

        for children in self.children.iter_mut() {
            for child in children.iter_mut() {
                let child = child.clone();

                tasks.spawn(async move {
                    let mut child = child.lock().await;
                    child.commit().await
                });
            }

            while let Some(result) = tasks.join_next().await {
                match result {
                    Ok(r) => {
                        if let Err(err) = r {
                            errors.push(crate::Error::Transaction(Box::new(err)));
                        }
                    }
                    Err(err) => errors.push(crate::Error::from(err)),
                };
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(crate::Error::RootTransaction(errors))
        }
    }

    async fn rollback(&mut self) -> Result<(), crate::Error> {
        let mut tasks = tokio::task::JoinSet::new();
        let mut errors = Vec::with_capacity(self.children.len());

        for children in self.children.iter_mut() {
            for child in children.iter_mut() {
                let child = child.clone();

                tasks.spawn(async move {
                    let mut child = child.lock().await;
                    child.rollback().await
                });
            }

            while let Some(result) = tasks.join_next().await {
                match result {
                    Ok(r) => {
                        if let Err(err) = r {
                            errors.push(crate::Error::Transaction(Box::new(err)));
                        }
                    }
                    Err(err) => errors.push(crate::Error::from(err)),
                };
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(crate::Error::RootTransaction(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transactions::Transaction;

    use super::*;

    use std::sync::{Arc, Mutex};

    struct ChildTransaction {
        value: u8,
        test_vec: Arc<Mutex<Vec<u8>>>,
    }

    #[async_trait::async_trait]
    impl crate::transactions::Transaction for ChildTransaction {
        async fn commit(&mut self) -> Result<(), crate::Error> {
            self.test_vec.lock().unwrap().push(self.value);

            Ok(())
        }

        async fn rollback(&mut self) -> Result<(), crate::Error> {
            let mut test_vec = self.test_vec.lock().unwrap();

            let index = test_vec.iter().position(|v| v == &self.value).unwrap();
            test_vec.remove(index);

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_add_children_sequential() {
        let test_vec = Arc::new(Mutex::new(Vec::new()));

        let transaction_a = ChildTransaction {
            value: 1,
            test_vec: test_vec.clone(),
        };
        let transaction_b = ChildTransaction {
            value: 2,
            test_vec: test_vec.clone(),
        };
        let transaction_c = ChildTransaction {
            value: 3,
            test_vec: test_vec.clone(),
        };

        let mut root_transaction = RootTransaction::new();
        root_transaction.add_child(transaction_a);
        root_transaction.add_child(transaction_b);
        root_transaction.add_child(transaction_c);

        root_transaction.commit().await.unwrap();

        {
            let unwrapped_v = test_vec.lock().unwrap();
            let v: &[u8] = unwrapped_v.as_ref();

            assert_eq!(v, &[1, 2, 3]);
        }

        root_transaction.rollback().await.unwrap();

        {
            let unwrapped_v = test_vec.lock().unwrap();
            let v: &[u8] = unwrapped_v.as_ref();

            assert!(v.is_empty());
        }
    }

    #[tokio::test]
    async fn test_add_children_parallel() {
        let test_vec = Arc::new(Mutex::new(Vec::new()));

        let transaction_a = ChildTransaction {
            value: 1,
            test_vec: test_vec.clone(),
        };
        let transaction_b = ChildTransaction {
            value: 2,
            test_vec: test_vec.clone(),
        };
        let transaction_c = ChildTransaction {
            value: 3,
            test_vec: test_vec.clone(),
        };

        let mut root_transaction = RootTransaction::new();
        root_transaction.add_child_parallel(transaction_a);
        root_transaction.add_child_parallel(transaction_b);
        root_transaction.add_child_parallel(transaction_c);

        root_transaction.commit().await.unwrap();

        {
            let unwrapped_v = test_vec.lock().unwrap();
            let mut v: Vec<u8> = unwrapped_v.clone();
            v.sort_unstable();

            assert_eq!(v, &[1, 2, 3]);
        }

        root_transaction.rollback().await.unwrap();

        {
            let unwrapped_v = test_vec.lock().unwrap();
            let v: &[u8] = unwrapped_v.as_ref();

            assert!(v.is_empty());
        }
    }
}
