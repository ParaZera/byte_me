use crate::app::{DataType, Endianness};

pub trait ByteConverter {
    fn convert(&self, value: u128, data_type: DataType, endianness: Endianness) -> String;
    fn name(&self) -> &'static str;
}

struct LittleEndianBytes;
struct BigEndianBytes;
struct DecimalConverter;
struct HexConverter;
struct BinaryConverter;
struct OctalConverter;
struct FloatConverter;

impl ByteConverter for LittleEndianBytes {
    fn convert(&self, value: u128, data_type: DataType, _: Endianness) -> String {
        let bytes = match data_type {
            DataType::U8 => vec![(value as u8)],
            DataType::U16 => (value as u16).to_le_bytes().to_vec(),
            DataType::U32 => (value as u32).to_le_bytes().to_vec(),
            DataType::U64 => (value as u64).to_le_bytes().to_vec(),
            DataType::U128 => value.to_le_bytes().to_vec(),
            DataType::I8 => vec![(value as i8 as u8)],
            DataType::I16 => (value as i16).to_le_bytes().to_vec(),
            DataType::I32 => (value as i32).to_le_bytes().to_vec(),
            DataType::I64 => (value as i64).to_le_bytes().to_vec(),
            DataType::I128 => (value as i128).to_le_bytes().to_vec(),
            DataType::F32 => (f32::from_bits(value as u32)).to_le_bytes().to_vec(),
            DataType::F64 => (f64::from_bits(value as u64)).to_le_bytes().to_vec(),
        };

        format!("{:02X?}", bytes)
    }

    fn name(&self) -> &'static str {
        "Little Endian Bytes"
    }
}

impl ByteConverter for BigEndianBytes {
    fn convert(&self, value: u128, data_type: DataType, _: Endianness) -> String {
        let bytes = match data_type {
            DataType::U8 => vec![(value as u8)],
            DataType::U16 => (value as u16).to_be_bytes().to_vec(),
            DataType::U32 => (value as u32).to_be_bytes().to_vec(),
            DataType::U64 => (value as u64).to_be_bytes().to_vec(),
            DataType::U128 => value.to_be_bytes().to_vec(),
            DataType::I8 => vec![(value as i8 as u8)],
            DataType::I16 => (value as i16).to_be_bytes().to_vec(),
            DataType::I32 => (value as i32).to_be_bytes().to_vec(),
            DataType::I64 => (value as i64).to_be_bytes().to_vec(),
            DataType::I128 => (value as i128).to_be_bytes().to_vec(),
            DataType::F32 => (f32::from_bits(value as u32)).to_be_bytes().to_vec(),
            DataType::F64 => (f64::from_bits(value as u64)).to_be_bytes().to_vec(),
        };

        format!("{:02X?}", bytes)
    }

    fn name(&self) -> &'static str {
        "Big Endian Bytes"
    }
}

impl ByteConverter for FloatConverter {
    fn convert(&self, value: u128, data_type: DataType, _: Endianness) -> String {
        match data_type {
            DataType::F32 => format!("{}", f32::from_bits(value as u32)),
            DataType::F64 => format!("{}", f64::from_bits(value as u64)),
            _ => "Not a float type".to_string(),
        }
    }

    fn name(&self) -> &'static str {
        "Float Value"
    }
}

// Get all conversion results for display
pub fn get_conversions(
    value: u128,
    data_type: DataType,
    endianness: Endianness,
) -> Vec<(String, String)> {
    let converters: Vec<Box<dyn ByteConverter>> = vec![
        Box::new(LittleEndianBytes),
        Box::new(BigEndianBytes),
        Box::new(FloatConverter),
        // Add more converters as needed
    ];

    converters
        .iter()
        .map(|converter| {
            (
                converter.name().to_string(),
                converter.convert(value, data_type, endianness),
            )
        })
        .collect()
}

// Add a trait for custom converters that users can implement
pub trait CustomByteConverter {
    fn convert(&self, value: u128, data_type: DataType, endianness: Endianness) -> String;
    fn name(&self) -> &'static str;
}
