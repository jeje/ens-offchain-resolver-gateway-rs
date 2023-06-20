CREATE TYPE RECORD_TYPE AS ENUM ('address', 'text', 'content_hash');

CREATE TABLE ens_records (
  id SERIAL PRIMARY KEY,
  -- Would be `user.domain.eth` if the full ENS domain is `user.domain.eth`
  domain TEXT NOT NULL,
  -- The kind of record
  record_type RECORD_TYPE NOT NULL,
  -- Either an address or null if not a `AddressRecord` or `AddressBytesRecord`
  address_record_coin_type BIGINT,
  address_record_value TEXT,
  -- Either some text or null if not a `AddressRecord` or `AddressBytesRecord`
  text_record_key TEXT,
  text_record_value TEXT,
  -- Either some text or null if not a `ContentHashRecord`
  content_hash_record TEXT
);