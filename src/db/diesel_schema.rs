// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "record_type"))]
    pub struct RecordType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RecordType;

    ens_records (id) {
        id -> Int4,
        domain -> Text,
        record_type -> RecordType,
        address_record_coin_type -> Nullable<Int8>,
        address_record_value -> Nullable<Text>,
        text_record_key -> Nullable<Text>,
        text_record_value -> Nullable<Text>,
        content_hash_record -> Nullable<Text>,
    }
}
