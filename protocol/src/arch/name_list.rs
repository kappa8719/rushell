use bincode::de::Decoder;
use bincode::de::read::Reader;
use bincode::enc::Encoder;
use bincode::enc::write::Writer;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};

#[derive(BorrowDecode)]
pub struct NameList(Vec<String>);

impl NameList {
    pub fn new(list: Vec<String>) -> NameList {
        Self(list)
    }

    fn as_bytes(&self) -> Vec<u8> {
        let joined = self.0.join(",");
        joined.as_bytes().to_vec()
    }

    fn from_bytes(value: Vec<u8>) -> Self {
        Self::from(
            String::from_utf8(value)
                .unwrap()
                .split(",")
                .collect::<Vec<_>>(),
        )
    }
}
impl From<Vec<&str>> for NameList {
    fn from(value: Vec<&str>) -> Self {
        NameList::new(value.iter().map(|v| v.to_string()).collect())
    }
}

impl bincode::Encode for NameList {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let bytes = self.as_bytes();
        (bytes.len() as u32).encode(encoder)?;
        encoder.writer().write(self.as_bytes().as_slice())?;

        Ok(())
    }
}

impl<Context> bincode::Decode<Context> for NameList {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = usize::decode(decoder)?;

        decoder.claim_container_read::<u8>(len)?;
        let mut vec = vec![0u8; len];
        decoder.reader().read(&mut vec)?;

        Ok(NameList::from_bytes(vec))
    }
}

#[cfg(test)]
mod tests {
    use crate::arch::NameList;

    #[test]
    fn test_encode() {
        // use fixed int encoding network byte order
        let config = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();

        let empty = NameList::from(vec![]);
        let list1 = NameList::from(vec!["zlib"]);
        let list2 = NameList::from(vec!["zlib", "none"]);

        assert_eq!(
            bincode::encode_to_vec(empty, config).unwrap(),
            vec![0x0, 0x0, 0x0, 0x0]
        );
        assert_eq!(
            bincode::encode_to_vec(list1, config).unwrap(),
            vec![0x0, 0x0, 0x0, 0x4, 0x7a, 0x6c, 0x69, 0x62]
        );
        assert_eq!(
            bincode::encode_to_vec(list2, config).unwrap(),
            vec![
                0x0, 0x0, 0x0, 0x9, 0x7a, 0x6c, 0x69, 0x62, 0x2c, 0x6e, 0x6f, 0x6e, 0x65
            ]
        );
    }
}
