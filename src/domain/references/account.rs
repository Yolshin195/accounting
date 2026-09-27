use crate::domain::reference::{FieldDescriptor, ReferenceDescriptor};
use crate::domain::references::currency;

pub const KIND: &str = "accounts";

pub fn descriptor() -> ReferenceDescriptor {
    ReferenceDescriptor::new(KIND, "accounting_accounts", "Счета", "wallet")
        .field(
            FieldDescriptor::reference("currency", "currency_id", "Валюта", &currency::descriptor())
                .required(),
        )
        .field(
            FieldDescriptor::enumeration(
                "accountType",
                "account_type",
                "Тип счёта",
                &[("CASH", "Наличные"), ("CARD", "Карта"), ("DEPOSIT", "Депозит")],
            )
            .required(),
        )
        .field(
            FieldDescriptor::decimal("initialBalance", "initial_balance", "Начальный остаток")
                .required()
                .default_value(0),
        )
        .list_columns(&["code", "name", "currency", "accountType"])
}
