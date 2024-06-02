CREATE TABLE IF NOT EXISTS warheadfactory_warhead_claimed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "claimed_at" DECIMAL,
    "claimer" VARCHAR(40),
    "warhead_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_created (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "dropper" VARCHAR(40),
    "warhead_address" VARCHAR(40),
    "warhead_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_created_with_receiver (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "dropper" VARCHAR(40),
    "target_receiver" VARCHAR(40),
    "warhead_address" VARCHAR(40),
    "warhead_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS warheadfactory_warhead_dropped (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "impact_time" DECIMAL,
    "target_lat" DECIMAL,
    "target_long" DECIMAL,
    "warhead_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);

