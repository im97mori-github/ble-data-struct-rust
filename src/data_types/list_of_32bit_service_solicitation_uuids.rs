//! List of 32-bit Service Solicitation UUIDs (Data Type Value: 0x1f) module.

use uuid::Uuid;

use crate::{data_types::data_type::DataType, BASE_UUID};

/// List of 32-bit Service Solicitation UUIDs.

#[derive(Debug, PartialEq, Clone)]
pub struct ListOf32BitServiceSolicitationUUIDs {
    /// data length
    pub length: u8,

    /// UUIDs
    pub uuids: Vec<Uuid>,
}

impl ListOf32BitServiceSolicitationUUIDs {
    /// Create [`ListOf32BitServiceSolicitationUUIDs`] from [`Vec<Uuid>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuids: Vec<Uuid> = [
    ///     uuid!("00000001-0000-1000-8000-00805F9B34FB"),
    ///     uuid!("00000002-0000-1000-8000-00805F9B34FB"),
    /// ]
    /// .to_vec();
    /// let result = ListOf32BitServiceSolicitationUUIDs::new(&uuids);
    /// assert_eq!(uuids.len() as u8 * 4 + 1, result.length);
    /// assert_eq!(uuids, result.uuids);
    /// ```
    pub fn new(uuids: &Vec<Uuid>) -> Self {
        Self {
            length: (uuids.len() * 4 + 1) as u8,
            uuids: uuids.clone(),
        }
    }
}

impl TryFrom<&Vec<u8>> for ListOf32BitServiceSolicitationUUIDs {
    type Error = String;
    /// Create [`ListOf32BitServiceSolicitationUUIDs`] from [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(4)
    ///     .step_by(4)
    ///     .map(|f| {
    ///         let mut uuid_bytes_le: Vec<u8> = f[0..4].to_vec();
    ///         uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
    ///         Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap())
    ///     })
    ///     .collect();
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf32BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let result = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(uuids, data_type.uuids);
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 6 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        Ok(Self {
            length,
            uuids: value[2..2 + length as usize - 1]
                .windows(4)
                .step_by(4)
                .map(|w| {
                    let mut bytes = BASE_UUID.to_bytes_le();
                    bytes[0] = w[0];
                    bytes[1] = w[1];
                    bytes[2] = w[2];
                    bytes[3] = w[3];
                    Uuid::from_bytes_le(bytes)
                })
                .collect(),
        })
    }
}

impl Into<Vec<u8>> for ListOf32BitServiceSolicitationUUIDs {
    /// Create [`Vec<u8>`] from [`ListOf32BitServiceSolicitationUUIDs`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::{BASE_UUID, data_types::{list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [
    ///     0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
    /// ]
    /// .to_vec();
    /// let uuids: Vec<Uuid> = uuid_bytes
    ///     .windows(4)
    ///     .step_by(4)
    ///     .map(|f| {
    ///         let mut uuid_bytes_le: Vec<u8> = f[0..4].to_vec();
    ///         uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
    ///         Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap())
    ///     })
    ///     .collect();
    /// let result1 = ListOf32BitServiceSolicitationUUIDs::new(&uuids);
    ///
    /// let length = uuid_bytes.len() as u8 + 1;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ListOf32BitServiceSolicitationUUIDs::data_type());
    /// data.append(&mut uuid_bytes.clone());
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(
            &mut self
                .uuids
                .clone()
                .iter()
                .flat_map(|f| f.to_bytes_le()[..4].to_vec())
                .collect(),
        );
        return data;
    }
}

impl DataType for ListOf32BitServiceSolicitationUUIDs {
    /// return `0x1f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{list_of_32bit_service_solicitation_uuids::ListOf32BitServiceSolicitationUUIDs, data_type::DataType};
    ///
    /// assert_eq!(0x1f, ListOf32BitServiceSolicitationUUIDs::data_type());
    /// ```
    fn data_type() -> u8 {
        0x1f
    }
}

/// check `List of 32-bit Service Solicitation UUIDs.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::list_of_32bit_service_solicitation_uuids::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_list_of_32bit_service_solicitation_uuids(0x1f));
/// assert!(!is_list_of_32bit_service_solicitation_uuids(0x00));
/// ```
pub fn is_list_of_32bit_service_solicitation_uuids(data_type: u8) -> bool {
    ListOf32BitServiceSolicitationUUIDs::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::{
        data_types::{data_type::DataType, list_of_32bit_service_solicitation_uuids::*},
        BASE_UUID,
    };

    #[test]
    fn test_new() {
        let uuids: Vec<Uuid> = [
            uuid!("00000001-0000-1000-8000-00805F9B34FB"),
            uuid!("00000002-0000-1000-8000-00805F9B34FB"),
        ]
        .to_vec();
        let result = ListOf32BitServiceSolicitationUUIDs::new(&uuids);
        assert_eq!(uuids.len() as u8 * 4 + 1, result.length);
        assert_eq!(uuids, result.uuids);
    }

    #[test]
    fn test_try_from() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(4)
            .step_by(4)
            .map(|f| {
                let mut uuid_bytes_le: Vec<u8> = f[0..4].to_vec();
                uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
                Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap())
            })
            .collect();
        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf32BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let result = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(uuids, data_type.uuids);

        let mut data: Vec<u8> = vec![0u8; 5];
        data[0] = data.len() as u8 - 1;
        let result = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [
            0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8,
        ]
        .to_vec();
        let uuids: Vec<Uuid> = uuid_bytes
            .windows(4)
            .step_by(4)
            .map(|f| {
                let mut uuid_bytes_le: Vec<u8> = f[0..4].to_vec();
                uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
                Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap())
            })
            .collect();
        let result1 = ListOf32BitServiceSolicitationUUIDs::new(&uuids);

        let length = uuid_bytes.len() as u8 + 1;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ListOf32BitServiceSolicitationUUIDs::data_type());
        data.append(&mut uuid_bytes.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ListOf32BitServiceSolicitationUUIDs::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x1f, ListOf32BitServiceSolicitationUUIDs::data_type());
    }

    #[test]
    fn test_is_list_of_32bit_service_solicitation_uuids() {
        assert!(is_list_of_32bit_service_solicitation_uuids(0x1f));
        assert!(!is_list_of_32bit_service_solicitation_uuids(0x00));
    }
}
