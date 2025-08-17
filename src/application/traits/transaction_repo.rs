use crate::domain::transaction::{CreateTransaction, Transaction};
use async_trait::async_trait;
use uuid::Uuid;
use crate::application::dtos::pagination_dto::Pagination;

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn save(&self, transaction: CreateTransaction) -> anyhow::Result<Transaction>;
    async fn find_all(&self, user_id: Uuid, pagination: &Pagination) -> anyhow::Result<Vec<Transaction>>;
    async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<Transaction>;
    async fn count(&self, user_id: Uuid) -> anyhow::Result<i64>;
    async fn delete(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}

#[cfg(test)]
pub mod mock {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex}; // ToDo: Использовать асинхронный Mutex
    use chrono::{Utc};
    use rust_decimal::Decimal;
    use crate::domain::transaction::TransactionType;
    use super::*;

    #[derive(Clone)]
    pub struct InMemoryTransactionRepo {
        rows : Arc<Mutex<HashMap<Uuid, HashMap<Uuid, Transaction>>>>
    }

    impl InMemoryTransactionRepo {
        pub fn new() -> Self {
            Self {
                rows: Arc::new(Mutex::new(HashMap::new()))
            }
        }
    }

    #[async_trait]
    impl TransactionRepository for InMemoryTransactionRepo {
        async fn save(&self, transaction: CreateTransaction) -> anyhow::Result<Transaction> {
            let transaction_id = transaction.id; // Предполагаем, что у Transaction есть поле id
            let user_id = transaction.user_id;   // Предполагаем, что у Transaction есть поле user_id
            let new_transaction = Transaction {
                id: transaction.id,
                user_id: transaction.user_id,
                category_code: "FOOD".to_string(),
                amount: transaction.amount,
                description: transaction.description,
                created_at: transaction.created_at,
                transaction_type: transaction.transaction_type
            };

            // Блокируем mutex и работаем с данными
            {
                let mut rows = self.rows.lock()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;

                // Получаем или создаем HashMap для пользователя
                let user_transactions = rows.entry(user_id).or_insert_with(HashMap::new);
                
                // Сохраняем транзакцию
                user_transactions.insert(transaction_id, new_transaction.clone());
            } // Mutex автоматически освобождается здесь

            Ok(new_transaction)
        }

        async fn find_all(&self, user_id: Uuid, pagination: &Pagination) -> anyhow::Result<Vec<Transaction>> {
            let transactions = {
                let rows = self.rows.lock()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;

                // Получаем транзакции пользователя или пустой HashMap
                rows.get(&user_id)
                    .map(|user_transactions| user_transactions.values().cloned().collect::<Vec<_>>())
                    .unwrap_or_default()
            }; // Mutex освобождается здесь

            Ok(
                transactions
                    .into_iter()
                    .skip(pagination.offset() as usize)
                    .take(pagination.size as usize)
                    .collect()
            )
        }

        async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<Transaction> {
            todo!()
        }

        async fn count(&self, user_id: Uuid) -> anyhow::Result<i64> {
            let count = {
                let rows = self.rows.lock()
                    .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;
                
                rows.get(&user_id).map(|user_transactions| user_transactions.len() as i64).unwrap_or(0)
            };
            
            Ok(count)       
        }

        async fn delete(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_save() {
        // Given
        let repo = InMemoryTransactionRepo::new();
        let user_id = Uuid::new_v4();
        let transaction = CreateTransaction {
            id: Uuid::new_v4(),
            user_id,
            amount: Decimal::new(100, 0),
            category_id: Uuid::new_v4(),
            description: Some("FOOD category".to_string()),
            created_at: Utc::now().naive_utc(),
            transaction_type: TransactionType::Expense
        };

        // When
        repo.save(transaction.clone()).await.unwrap();

        // Then
        let rows = repo.find_all(user_id, &Pagination::default()).await.unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, transaction.id);
        assert_eq!(rows[0].user_id, transaction.user_id);
        assert_eq!(rows[0].amount, transaction.amount);
        assert_eq!(rows[0].description, transaction.description);
    }
}