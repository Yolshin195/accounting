use crate::domain::reference::{FieldDescriptor, ReferenceDescriptor};

pub const KIND: &str = "currencies";

pub fn descriptor() -> ReferenceDescriptor {
    ReferenceDescriptor::new(KIND, "accounting_currencies", "Валюты", "currency")
        .field(FieldDescriptor::string("symbol", "symbol", "Символ").max_length(8))
        .field(
            FieldDescriptor::integer("decimals", "decimals", "Знаков после запятой")
                .required()
                .default_value(2),
        )
        .field(FieldDescriptor::boolean("isBase", "is_base", "Основная валюта"))
        .list_columns(&["code", "name", "symbol", "isBase"])
}
