use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum QR {
    #[deku(id = "0")]
    Query,
    #[deku(id = "1")]
    Reply,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum OPCODE {
    #[deku(id = "0")]
    Query,
    #[deku(id = "1")]
    IQuery,
    #[deku(id = "2")]
    Status,
    #[deku(id_pat = "3..=7")]
    Unknown(u8),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian"
)]
pub enum RCODE {
    #[deku(id = "0")]
    NoError,
    #[deku(id = "1")]
    FormErr,
    #[deku(id = "2")]
    ServFail,
    #[deku(id = "3")]
    NXDomain,
    #[deku(id = "4")]
    NotImp,
    #[deku(id = "5")]
    Refused,
    #[deku(id = "6")]
    YXDomain,
    #[deku(id = "7")]
    YXRRSet,
    #[deku(id = "8")]
    NXRRSet,
    #[deku(id = "9")]
    NotAuth,
    #[deku(id = "10")]
    NotZone,
    #[deku(id = "11")]
    DSOTypeNI,
    #[deku(id_pat = "12..=15")]
    Unassigned(u8),
    // #[deku(id = "16")]
    // BADSIG,
    // #[deku(id = "17")]
    // BADKEY,
    // #[deku(id = "18")]
    // BADTIME,
    // #[deku(id = "19")]
    // BADMODE,
    // #[deku(id = "20")]
    // BADNAME,
    // #[deku(id = "21")]
    // BADALG,
    // #[deku(id = "22")]
    // BADTRUNC,
    // #[deku(id = "23")]
    // BADCOOKIE,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Flags {
    pub qr: QR,
    pub opcode: OPCODE,
    #[deku(bits = "1")]
    pub aa: bool,
    #[deku(bits = "1")]
    pub tc: bool,
    #[deku(bits = "1")]
    pub rd: bool,
    #[deku(bits = "1")]
    pub ra: bool,
    #[deku(bits = "3")]
    pub z: u8,
    pub rcode: RCODE,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Header {
    pub id: u16,
    pub flags: Flags,

    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct Octets {
    #[deku(update = "self.data.len()")]
    count: u8,
    #[deku(count = "count")]
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data: Vec<u8> = vec![
            0x0D, 0x76, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];

        let (rest, val) = Header::from_bytes((&data, 0)).expect("Expected header");
        assert_eq!(0, rest.0.len());
        assert_eq!(0, rest.1);
        let expected = Header {
            id: 3446,
            flags: Flags {
                qr: QR::Query,
                opcode: OPCODE::Query,
                aa: false,
                tc: false,
                rd: true,
                ra: false,
                z: 2,
                rcode: RCODE::NoError,
            },
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 1,
        };
        assert_eq!(expected, val);

        // let data: Vec<u8> = vec![0b10100000];

        // let (rest, val) = QR::from_bytes((data.as_ref(), 0)).unwrap();
        // assert_eq!(QR::Reply, val);
        // assert_eq!(rest, (data.as_ref(), 1));

        // let data: Vec<u8> = vec![0b00100000];

        // let (rest, val) = OPCODE::from_bytes((data.as_ref(), 0)).unwrap();
        // assert_eq!(OPCODE::Status, val);
        // assert_eq!(rest, (data.as_ref(), 4));
    }
}
