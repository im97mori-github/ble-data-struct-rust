//! Data type parser module.

use super::{
    advertising_interval::{is_advertising_interval, AdvertisingInterval},
    advertising_interval_long::{is_advertising_interval_long, AdvertisingIntervalLong},
    appearance::{is_appearance, Appearance},
    big_info::{is_big_info, BigInfo},
    broadcast_code::{is_broadcast_code, BroadcastCode},
};

/// Data type parse result.
pub enum DataTypeParseResult {
    /// [`AdvertisingInterval`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalResult(Result<AdvertisingInterval, String>),

    /// [`AdvertisingIntervalLong`]'s [`TryFrom::try_from`] result.
    AdvertisingIntervalLongResult(Result<AdvertisingIntervalLong, String>),

    /// [`Appearance`]'s [`TryFrom::try_from`] result.
    AppearanceResult(Result<Appearance, String>),

    /// [`BigInfo`]'s [`TryFrom::try_from`] result.
    BigInfoResult(Result<BigInfo, String>),

    /// [`BroadcastCode`]'s [`TryFrom::try_from`] result.
    BroadcastCodeResult(Result<BroadcastCode, String>),

    /// Occurs for unsupported data types.
    DataTypeParseErr(String),
}

impl DataTypeParseResult {
    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    /// ```
    pub fn is_advertising_interval(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AdvertisingIntervalLongResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval_long::AdvertisingIntervalLong, parser::DataTypeParseResult};
    ///
    /// let advertising_interval_long: u32 = 0x01020304u32;
    /// let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
    /// assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    /// ```
    pub fn is_advertising_interval_long(&self) -> bool {
        matches!(self, DataTypeParseResult::AdvertisingIntervalLongResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::AppearanceResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{appearance::Appearance, parser::DataTypeParseResult};
    ///
    /// let appearance: u16 = 0x1444;
    /// let data: Vec<u8> = Appearance::new(appearance).into();
    /// assert!(DataTypeParseResult::from(&data).is_appearance());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_appearance());
    /// ```
    pub fn is_appearance(&self) -> bool {
        matches!(self, DataTypeParseResult::AppearanceResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::BigInfoResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{big_info::BigInfo, parser::DataTypeParseResult};
    ///
    /// let big_offset: u16 = 1;
    /// let big_offset_units: bool = true;
    /// let iso_interval: u16 = 2;
    /// let num_bis: u8 = 3;
    /// let nse: u8 = 4;
    /// let bn: u8 = 5;
    /// let sub_interval: u32 = 6;
    /// let pto: u8 = 7;
    /// let bis_spacing: u32 = 8;
    /// let irc: u8 = 9;
    /// let max_pdu: u8 = 10;
    /// let rfu: u8 = 11;
    /// let seed_access_address: u32 = 12;
    /// let sdu_interval: u32 = 13;
    /// let max_sdu: u16 = 14;
    /// let base_crc_init: u16 = 15;
    /// let ch_m: u64 = 16;
    /// let phy: u8 = 17;
    /// let bis_payload_count: u64 = 18;
    /// let framing: bool = false;
    /// let giv: Option<[u8; 8]> = None;
    /// let gskd: Option<[u8; 16]> = None;
    /// let data: Vec<u8> = BigInfo::new(
    ///     big_offset,
    ///     big_offset_units,
    ///     iso_interval,
    ///     num_bis,
    ///     nse,
    ///     bn,
    ///     sub_interval,
    ///     pto,
    ///     bis_spacing,
    ///     irc,
    ///     max_pdu,
    ///     rfu,
    ///     seed_access_address,
    ///     sdu_interval,
    ///     max_sdu,
    ///     base_crc_init,
    ///     ch_m,
    ///     phy,
    ///     bis_payload_count,
    ///     framing,
    ///     giv,
    ///     gskd,
    /// )
    /// .into();
    /// assert!(DataTypeParseResult::from(&data).is_big_info());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_big_info());
    /// ```
    pub fn is_big_info(&self) -> bool {
        matches!(self, DataTypeParseResult::BigInfoResult(_))
    }

    /// Returns `true` if the result is [`DataTypeParseResult::BroadcastCodeResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{broadcast_code::BroadcastCode, parser::DataTypeParseResult};
    ///
    /// let broadcast_code = [0x00u8; 4].to_vec();
    /// let data: Vec<u8> = BroadcastCode::new(&broadcast_code).into();
    /// assert!(DataTypeParseResult::from(&data).is_broadcast_code());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// assert!(!DataTypeParseResult::from(&data).is_broadcast_code());
    /// ```
    pub fn is_broadcast_code(&self) -> bool {
        matches!(self, DataTypeParseResult::BroadcastCodeResult(_))
    }
}

impl From<&Vec<u8>> for DataTypeParseResult {
    /// Create [`DataTypeParseResult`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::DataTypeParseResult};
    ///
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::AdvertisingIntervalResult(_)
    /// ));
    /// let data: Vec<u8> = Vec::new();
    /// assert!(matches!(
    ///     DataTypeParseResult::from(&data),
    ///     DataTypeParseResult::DataTypeParseErr(_)
    /// ));
    fn from(value: &Vec<u8>) -> Self {
        if let Some(data_type) = value.get(1) {
            if is_advertising_interval(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalResult(AdvertisingInterval::try_from(value))
            } else if is_advertising_interval_long(data_type.to_owned()) {
                DataTypeParseResult::AdvertisingIntervalLongResult(
                    AdvertisingIntervalLong::try_from(value),
                )
            } else if is_appearance(data_type.to_owned()) {
                DataTypeParseResult::AppearanceResult(Appearance::try_from(value))
            } else if is_big_info(data_type.to_owned()) {
                DataTypeParseResult::BigInfoResult(BigInfo::try_from(value))
            } else if is_broadcast_code(data_type.to_owned()) {
                DataTypeParseResult::BroadcastCodeResult(BroadcastCode::try_from(value))
            } else {
                DataTypeParseResult::DataTypeParseErr(
                    format!("Unknown data type :{}", data_type).to_string(),
                )
            }
        } else {
            DataTypeParseResult::DataTypeParseErr("Invalid data size".to_string())
        }
    }
}

/// Data types parse results.
pub struct DataTypeParseResults {
    /// Parse results.
    pub results: Vec<DataTypeParseResult>,
}

impl From<&Vec<Vec<u8>>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<Vec<u8>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
    /// vec.push(data);
    /// vec.push(vec![]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<Vec<u8>>) -> Self {
        Self {
            results: value
                .iter()
                .map(|f| DataTypeParseResult::from(f))
                .collect::<Vec<DataTypeParseResult>>(),
        }
    }
}

impl From<&Vec<u8>> for DataTypeParseResults {
    /// Create [`DataTypeParseResults`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{advertising_interval::AdvertisingInterval, parser::{DataTypeParseResult, DataTypeParseResults}};
    ///
    /// let mut vec: Vec<Vec<u8>> = Vec::new();
    /// let advertising_interval = 0x01;
    /// vec.push(AdvertisingInterval::new(advertising_interval).into());
    /// vec.push(vec![100]);
    ///
    /// let results = DataTypeParseResults::from(&vec);
    /// assert!(matches!(
    ///     results.results.get(0),
    ///     Some(DataTypeParseResult::AdvertisingIntervalResult(_))
    /// ));
    /// assert!(matches!(
    ///     results.results.get(1),
    ///     Some(DataTypeParseResult::DataTypeParseErr(_))
    /// ));
    /// assert!(matches!(results.results.get(2), None));
    /// ```
    fn from(value: &Vec<u8>) -> Self {
        let mut vec = Vec::new();
        let mut index = 0;
        let len = value.len();
        while index < len {
            let mut inner: Vec<u8> = Vec::new();
            let size = value[index];
            inner.append(&mut value[index..index + 1 + size as usize].to_vec());
            vec.push(inner);

            index += 1;
            index += size as usize;
        }
        Self::from(&vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::data_types::{
        advertising_interval::AdvertisingInterval,
        advertising_interval_long::AdvertisingIntervalLong, appearance::Appearance,
        big_info::BigInfo, broadcast_code::BroadcastCode, parser::DataTypeParseResult,
    };

    use super::DataTypeParseResults;

    #[test]
    fn test_is_advertising_interval() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval());
    }

    #[test]
    fn test_is_advertising_interval_long() {
        let advertising_interval_long: u32 = 0x01020304u32;
        let data: Vec<u8> = AdvertisingIntervalLong::new(true, advertising_interval_long).into();
        assert!(DataTypeParseResult::from(&data).is_advertising_interval_long());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_advertising_interval_long());
    }

    #[test]
    fn test_is_appearance() {
        let appearance: u16 = 0x1444;
        let data: Vec<u8> = Appearance::new(appearance).into();
        assert!(DataTypeParseResult::from(&data).is_appearance());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_appearance());
    }

    #[test]
    fn test_is_big_info() {
        let big_offset: u16 = 1;
        let big_offset_units: bool = true;
        let iso_interval: u16 = 2;
        let num_bis: u8 = 3;
        let nse: u8 = 4;
        let bn: u8 = 5;
        let sub_interval: u32 = 6;
        let pto: u8 = 7;
        let bis_spacing: u32 = 8;
        let irc: u8 = 9;
        let max_pdu: u8 = 10;
        let rfu: u8 = 11;
        let seed_access_address: u32 = 12;
        let sdu_interval: u32 = 13;
        let max_sdu: u16 = 14;
        let base_crc_init: u16 = 15;
        let ch_m: u64 = 16;
        let phy: u8 = 17;
        let bis_payload_count: u64 = 18;
        let framing: bool = false;
        let giv: Option<[u8; 8]> = None;
        let gskd: Option<[u8; 16]> = None;
        let data: Vec<u8> = BigInfo::new(
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        )
        .into();
        assert!(DataTypeParseResult::from(&data).is_big_info());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_big_info());
    }

    #[test]
    fn test_is_broadcast_code() {
        let broadcast_code = [0x00u8; 4].to_vec();
        let data: Vec<u8> = BroadcastCode::new(&broadcast_code).into();
        assert!(DataTypeParseResult::from(&data).is_broadcast_code());

        let data: Vec<u8> = Vec::new();
        assert!(!DataTypeParseResult::from(&data).is_broadcast_code());
    }
    #[test]
    fn test_result_from_vec() {
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::AdvertisingIntervalResult(_)
        ));

        let data: Vec<u8> = Vec::new();
        assert!(matches!(
            DataTypeParseResult::from(&data),
            DataTypeParseResult::DataTypeParseErr(_)
        ));
    }

    #[test]
    fn test_results_from_vec_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        let data: Vec<u8> = AdvertisingInterval::new(advertising_interval).into();
        vec.push(data);
        vec.push(vec![]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }

    #[test]
    fn test_results_from_vec() {
        let mut vec: Vec<Vec<u8>> = Vec::new();
        let advertising_interval = 0x01;
        vec.push(AdvertisingInterval::new(advertising_interval).into());
        vec.push(vec![100]);

        let results = DataTypeParseResults::from(&vec);
        assert!(matches!(
            results.results.get(0),
            Some(DataTypeParseResult::AdvertisingIntervalResult(_))
        ));
        assert!(matches!(
            results.results.get(1),
            Some(DataTypeParseResult::DataTypeParseErr(_))
        ));
        assert!(matches!(results.results.get(2), None));
    }
}
