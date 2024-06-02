CREATE TABLE IF NOT EXISTS warheadfactory_warhead_claimed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "claimed_at" UInt256,
    "claimer" VARCHAR(40),
    "warhead_id" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_created (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "dropper" VARCHAR(40),
    "warhead_address" VARCHAR(40),
    "warhead_id" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_created_with_receiver (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "dropper" VARCHAR(40),
    "target_receiver" VARCHAR(40),
    "warhead_address" VARCHAR(40),
    "warhead_id" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_dropped (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" UInt64,
    "impact_time" UInt256,
    "target_lat" Int256,
    "target_long" Int256,
    "warhead_id" UInt256
) ENGINE = MergeTree PRIMARY KEY ("evt_tx_hash","evt_index");

