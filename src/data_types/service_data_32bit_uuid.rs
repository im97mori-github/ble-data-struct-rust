//! Service Data - 32-bit UUID (Data Type Value: 0x20) module.

use uuid::Uuid;

use crate::{data_types::data_type::DataType, BASE_UUID};

/// Service Data - 32-bit UUID.
pub struct ServiceData32BitUUID {
    /// data length
    pub length: u8,

    /// UUID
    pub uuid: Uuid,

    /// Additional service data
    pub additional_service_data: Vec<u8>,
}

impl ServiceData32BitUUID {
    /// Create [ServiceData32BitUUID] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::service_data_32bit_uuid::ServiceData32BitUUID;
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
    /// let additional_service_data = [0x05u8].to_vec();
    /// let result = ServiceData32BitUUID::new(&uuid, &additional_service_data);
    /// assert_eq!(additional_service_data.len() as u8 + 5, result.length);
    /// assert_eq!(uuid, result.uuid);
    /// assert_eq!(additional_service_data, result.additional_service_data);
    /// ```
    pub fn new(uuid: &Uuid, additional_service_data: &Vec<u8>) -> Self {
        Self {
            length: 5 + additional_service_data.len() as u8,
            uuid: uuid.clone(),
            additional_service_data: additional_service_data.clone(),
        }
    }

    /// Create [ServiceData32BitUUID] from `Vec<u8>` with offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{service_data_32bit_uuid::ServiceData32BitUUID, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
    /// uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
    /// let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
    /// let additional_service_data = [0x05u8].to_vec();
    /// let length = additional_service_data.len() as u8 + 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ServiceData32BitUUID::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// data.append(&mut additional_service_data.clone());
    /// 
    /// let result = ServiceData32BitUUID::from_with_offset(&data, 0);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuid, result.uuid);
    /// assert_eq!(additional_service_data, result.additional_service_data);
    /// 
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(ServiceData32BitUUID::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// data.append(&mut additional_service_data.clone());
    /// let result = ServiceData32BitUUID::from_with_offset(&data, 1);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuid, result.uuid);
    /// assert_eq!(additional_service_data, result.additional_service_data);
    /// ```
    pub fn from_with_offset(data: &Vec<u8>, offset: usize) -> Self {
        let data = data[offset..].to_vec();
        let length = data[0];
        let mut bytes = BASE_UUID.to_bytes_le();
        bytes[0] = data[2];
        bytes[1] = data[3];
        bytes[2] = data[4];
        bytes[3] = data[5];
        Self {
            length,
            uuid: Uuid::from_bytes_le(bytes),
            additional_service_data: data[6..1 + length as usize].to_vec(),
        }
    }
}

impl From<&Vec<u8>> for ServiceData32BitUUID {
    /// Create [ServiceData32BitUUID] from `Vec<u8>`.
    ///
    /// [`ServiceData32BitUUID::from_with_offset`]
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{service_data_32bit_uuid::ServiceData32BitUUID, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
    /// uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
    /// let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
    /// let additional_service_data = [0x05u8].to_vec();
    /// let length = additional_service_data.len() as u8 + 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ServiceData32BitUUID::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// data.append(&mut additional_service_data.clone());
    /// 
    /// let result = ServiceData32BitUUID::from(&data);
    /// assert_eq!(length, result.length);
    /// assert_eq!(uuid, result.uuid);
    /// assert_eq!(additional_service_data, result.additional_service_data);
    /// ```
    fn from(data: &Vec<u8>) -> Self {
        Self::from_with_offset(data, 0)
    }
}

impl Into<Vec<u8>> for ServiceData32BitUUID {
    /// Create `Vec<u8>` from [ServiceData32BitUUID].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::{BASE_UUID, data_types::{service_data_32bit_uuid::ServiceData32BitUUID, data_type::DataType}};
    /// use uuid::{uuid, Uuid};
    ///
    /// let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
    /// let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
    /// uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
    /// let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
    /// let additional_service_data = [0x05u8].to_vec();
    /// let result1 = ServiceData32BitUUID::new(&uuid, &additional_service_data);
    /// 
    /// let length = additional_service_data.len() as u8 + 5;
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(ServiceData32BitUUID::data_type());
    /// data.append(&mut uuid_bytes.clone());
    /// data.append(&mut additional_service_data.clone());
    /// 
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    /// 
    /// let result2 = ServiceData32BitUUID::from(&data);
    /// let into_data: Vec<u8> = result2.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.append(&mut self.uuid.to_bytes_le()[..4].to_vec());
        data.append(&mut self.additional_service_data.clone());
        return data;
    }
}

impl DataType for ServiceData32BitUUID {
    /// return `0x20`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_class::data_types::{service_data_32bit_uuid::ServiceData32BitUUID, data_type::DataType};
    ///
    /// assert_eq!(0x20, ServiceData32BitUUID::data_type());
    /// ```
    fn data_type() -> u8 {
        0x20
    }
}

/// check `Service Data - 32-bit UUID.` data type.
///
/// # Examples
///
/// ```
/// use ble_data_class::data_types::service_data_32bit_uuid::*;
/// use ble_data_class::data_types::data_type::DataType;
///
/// assert!(is_service_data_32bit_uuid(0x20));
/// assert!(!is_service_data_32bit_uuid(0x00));
/// ```
pub fn is_service_data_32bit_uuid(data_type: u8) -> bool {
    ServiceData32BitUUID::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use uuid::{uuid, Uuid};

    use crate::{
        data_types::{data_type::DataType, service_data_32bit_uuid::*},
        BASE_UUID,
    };

    #[test]
    fn test_new() {
        let uuid = uuid!("04030201-0000-1000-8000-00805F9B34FB");
        let additional_service_data = [0x05u8].to_vec();
        let result = ServiceData32BitUUID::new(&uuid, &additional_service_data);
        assert_eq!(additional_service_data.len() as u8 + 5, result.length);
        assert_eq!(uuid, result.uuid);
        assert_eq!(additional_service_data, result.additional_service_data);
    }

    #[test]
    fn test_from_with_offset() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
        uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
        let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
        let additional_service_data = [0x05u8].to_vec();
        let length = additional_service_data.len() as u8 + 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ServiceData32BitUUID::data_type());
        data.append(&mut uuid_bytes.clone());
        data.append(&mut additional_service_data.clone());

        let result = ServiceData32BitUUID::from_with_offset(&data, 0);
        assert_eq!(length, result.length);
        assert_eq!(uuid, result.uuid);
        assert_eq!(additional_service_data, result.additional_service_data);

        data = Vec::new();
        data.push(0);
        data.push(length);
        data.push(ServiceData32BitUUID::data_type());
        data.append(&mut uuid_bytes.clone());
        data.append(&mut additional_service_data.clone());
        let result = ServiceData32BitUUID::from_with_offset(&data, 1);
        assert_eq!(length, result.length);
        assert_eq!(uuid, result.uuid);
        assert_eq!(additional_service_data, result.additional_service_data);
    }

    #[test]
    fn test_from() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
        uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
        let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
        let additional_service_data = [0x05u8].to_vec();
        let length = additional_service_data.len() as u8 + 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ServiceData32BitUUID::data_type());
        data.append(&mut uuid_bytes.clone());
        data.append(&mut additional_service_data.clone());

        let result = ServiceData32BitUUID::from(&data);
        assert_eq!(length, result.length);
        assert_eq!(uuid, result.uuid);
        assert_eq!(additional_service_data, result.additional_service_data);
    }

    #[test]
    fn test_into() {
        let uuid_bytes: Vec<u8> = [0x01u8, 0x02u8, 0x03u8, 0x04u8].to_vec();
        let mut uuid_bytes_le: Vec<u8> = uuid_bytes.to_vec();
        uuid_bytes_le.append(&mut BASE_UUID.to_bytes_le()[4..].to_vec());
        let uuid = Uuid::from_bytes_le(uuid_bytes_le.try_into().unwrap());
        let additional_service_data = [0x05u8].to_vec();
        let result1 = ServiceData32BitUUID::new(&uuid, &additional_service_data);

        let length = additional_service_data.len() as u8 + 5;
        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(ServiceData32BitUUID::data_type());
        data.append(&mut uuid_bytes.clone());
        data.append(&mut additional_service_data.clone());

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = ServiceData32BitUUID::from(&data);
        let into_data: Vec<u8> = result2.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x20, ServiceData32BitUUID::data_type());
    }

    #[test]
    fn test_is_service_data_32bit_uuid() {
        assert!(is_service_data_32bit_uuid(0x20));
        assert!(!is_service_data_32bit_uuid(0x00));
    }
}