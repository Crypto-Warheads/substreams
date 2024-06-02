// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub warheadfactory_warhead_claimeds: ::prost::alloc::vec::Vec<WarheadfactoryWarheadClaimed>,
    #[prost(message, repeated, tag="2")]
    pub warheadfactory_warhead_createds: ::prost::alloc::vec::Vec<WarheadfactoryWarheadCreated>,
    #[prost(message, repeated, tag="3")]
    pub warheadfactory_warhead_created_with_receivers: ::prost::alloc::vec::Vec<WarheadfactoryWarheadCreatedWithReceiver>,
    #[prost(message, repeated, tag="4")]
    pub warheadfactory_warhead_droppeds: ::prost::alloc::vec::Vec<WarheadfactoryWarheadDropped>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarheadfactoryWarheadClaimed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub warhead_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub claimer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub claimed_at: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarheadfactoryWarheadCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub warhead_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub dropper: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub warhead_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarheadfactoryWarheadCreatedWithReceiver {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub warhead_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub dropper: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub warhead_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub target_receiver: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarheadfactoryWarheadDropped {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub warhead_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub target_lat: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub target_long: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub impact_time: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
