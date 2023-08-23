//! BLE data class.
pub mod data_types {
    //! EIR/AD/SRD/ACAD/OOB module.
    pub mod data_type;
    pub mod complete_local_name;
    pub mod advertising_interval;
    pub mod advertising_interval_long;
    pub mod appearance;
    pub mod channel_map_update_indication;
    pub mod big_info;
}

pub use data_types::data_type::DataType;
pub use data_types::data_type::is_advertising_interval;
pub use data_types::data_type::is_advertising_interval_long;
pub use data_types::data_type::is_appearance;
pub use data_types::data_type::is_complete_local_name;
pub use data_types::data_type::is_channel_map_update_indication;
pub use data_types::data_type::is_big_info;
pub use data_types::complete_local_name::CompleteLocalName;
pub use data_types::advertising_interval::AdvertisingInterval;
pub use data_types::advertising_interval_long::AdvertisingIntervalLong;
pub use data_types::appearance::Appearance;
pub use data_types::channel_map_update_indication::ChannelMapUpdateIndication;
pub use data_types::big_info::BigInfo;