mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const WARHEADFACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("67744580e1ffc939e7dae8b9c7f3899b0667d8a8");

fn map_warheadfactory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.warheadfactory_warhead_claimeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WARHEADFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::warheadfactory_contract::events::WarheadClaimed::match_and_decode(log) {
                        return Some(contract::WarheadfactoryWarheadClaimed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            claimed_at: event.claimed_at.to_string(),
                            claimer: event.claimer,
                            warhead_id: event.warhead_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.warheadfactory_warhead_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WARHEADFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::warheadfactory_contract::events::WarheadCreated::match_and_decode(log) {
                        return Some(contract::WarheadfactoryWarheadCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            dropper: event.dropper,
                            warhead_address: event.warhead_address,
                            warhead_id: event.warhead_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.warheadfactory_warhead_created_with_receivers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WARHEADFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::warheadfactory_contract::events::WarheadCreatedWithReceiver::match_and_decode(log) {
                        return Some(contract::WarheadfactoryWarheadCreatedWithReceiver {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            dropper: event.dropper,
                            target_receiver: event.target_receiver,
                            warhead_address: event.warhead_address,
                            warhead_id: event.warhead_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.warheadfactory_warhead_droppeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WARHEADFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::warheadfactory_contract::events::WarheadDropped::match_and_decode(log) {
                        return Some(contract::WarheadfactoryWarheadDropped {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            impact_time: event.impact_time.to_string(),
                            target_lat: event.target_lat.to_string(),
                            target_long: event.target_long.to_string(),
                            warhead_id: event.warhead_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_warheadfactory_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.warheadfactory_warhead_claimeds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_claimed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("claimed_at", BigDecimal::from_str(&evt.claimed_at).unwrap())
            .set("claimer", Hex(&evt.claimer).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_createds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dropper", Hex(&evt.dropper).to_string())
            .set("warhead_address", Hex(&evt.warhead_address).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_created_with_receivers.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_created_with_receiver", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dropper", Hex(&evt.dropper).to_string())
            .set("target_receiver", Hex(&evt.target_receiver).to_string())
            .set("warhead_address", Hex(&evt.warhead_address).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_droppeds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_dropped", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("impact_time", BigDecimal::from_str(&evt.impact_time).unwrap())
            .set("target_lat", BigDecimal::from_str(&evt.target_lat).unwrap())
            .set("target_long", BigDecimal::from_str(&evt.target_long).unwrap())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
}


fn graph_warheadfactory_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.warheadfactory_warhead_claimeds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_claimed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("claimed_at", BigDecimal::from_str(&evt.claimed_at).unwrap())
            .set("claimer", Hex(&evt.claimer).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_createds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dropper", Hex(&evt.dropper).to_string())
            .set("warhead_address", Hex(&evt.warhead_address).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_created_with_receivers.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_created_with_receiver", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dropper", Hex(&evt.dropper).to_string())
            .set("target_receiver", Hex(&evt.target_receiver).to_string())
            .set("warhead_address", Hex(&evt.warhead_address).to_string())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
    events.warheadfactory_warhead_droppeds.iter().for_each(|evt| {
        tables
            .create_row("warheadfactory_warhead_dropped", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("impact_time", BigDecimal::from_str(&evt.impact_time).unwrap())
            .set("target_lat", BigDecimal::from_str(&evt.target_lat).unwrap())
            .set("target_long", BigDecimal::from_str(&evt.target_long).unwrap())
            .set("warhead_id", BigDecimal::from_str(&evt.warhead_id).unwrap());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_warheadfactory_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_warheadfactory_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_warheadfactory_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
