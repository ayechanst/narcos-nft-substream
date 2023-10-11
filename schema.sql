
CREATE TABLE IF NOT EXISTS approvals (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" DECIMAL,
    "evt_block_number" DECIMAL,
    "owner" VARCHAR(40),
    "spender" VARCHAR(40),
    "value" DECIMAL
);
CREATE TABLE IF NOT EXISTS max_tx_amount_updateds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" DECIMAL,
    "evt_block_number" DECIMAL,
    "u_max_tx_amount" DECIMAL
);
CREATE TABLE IF NOT EXISTS ownership_transferreds (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" DECIMAL,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40)
);
CREATE TABLE IF NOT EXISTS transfers (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" DECIMAL,
    "evt_block_number" DECIMAL,
    "from" VARCHAR(40),
    "to" VARCHAR(40),
    "value" DECIMAL
);