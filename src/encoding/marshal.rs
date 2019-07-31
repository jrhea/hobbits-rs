
use crate::encoding::EwpError;
use std::mem::transmute;
use byteorder::{BigEndian, WriteBytesExt};

pub use super::envelope::{Envelope};

/// Marshal takes a parsed message and encodes it to a wire protocol message
pub fn marshal(msg: &Envelope) -> Result<Vec<u8>, EwpError> {

    let mut outbytes: Vec<u8> = Envelope::preamble().into_bytes();
    outbytes.write_u32::<BigEndian>(msg.version).unwrap();
    outbytes.write_u8(msg.protocol).unwrap();
    outbytes.write_u32::<BigEndian>(msg.header.len() as u32).unwrap();
    outbytes.write_u32::<BigEndian>(msg.body.len() as u32).unwrap();
    outbytes.extend(&msg.header);
    outbytes.extend(&msg.body);

    return Ok(outbytes)
}



#[cfg(test)]
mod tests {
use bytes::BytesMut;
use super::{Envelope, marshal, EwpError};

    #[test]
    fn basic_sanity() {
        //   - desc: 'no body'
        //     marshalled: "EWP 3 0 0 0 "
        let mut marshalled = marshal(&Envelope::new(3, 0, &vec!(), &vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}",hex::encode("EWP"),"00000003000000000000000000"));
        //   - desc: '10 byte body'
        //     marshalled: "EWP 3 0 0 10  0123456789"
        marshalled = marshal(&Envelope::new(3, 0, &vec!(), "0123456789".as_bytes())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}{}",hex::encode("EWP"),"0000000300000000000000000a", hex::encode("0123456789".as_bytes())));
        //   - desc: '10 byte header'
        //     marshalled: "EWP 3 0 10 0 0123456789 "
        marshalled = marshal(&Envelope::new(3, 0, "0123456789".as_bytes(),&vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}{}",hex::encode("EWP"),"00000003000000000a00000000", hex::encode("0123456789".as_bytes())));
        //   - desc: '9 byte header, 10 byte body'
        //     marshalled: "EWP 3 0 9 10 987654321 0123456789"
        marshalled = marshal(&Envelope::new(3, 0, "987654321".as_bytes(),"0123456789".as_bytes())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}{}{}",hex::encode("EWP"),"0000000300000000090000000a", hex::encode("987654321".as_bytes()), hex::encode("0123456789".as_bytes())));
    }

    #[test]
    fn different_commands() {
        //   - desc: GOSSIP
        //     marshalled: "EWP 3 2 0 0"
        let mut marshalled = marshal(&Envelope::new(3, 1, &vec!(), &vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}",hex::encode("EWP"),"00000003010000000000000000"));
        //   - desc: PING
        //     marshalled: "EWP 3 2 0 0"
        marshalled = marshal(&Envelope::new(3, 2, &vec!(), &vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}",hex::encode("EWP"),"00000003020000000000000000"));
        //   - desc: UNDEFINED PROTOCOL
        //     marshalled: "EWP 3 3 0 0"
        marshalled = marshal(&Envelope::new(3, 3, &vec!(), &vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}",hex::encode("EWP"),"00000003030000000000000000"));
        //   - desc:  UNDEFINED PROTOCOL
        //     marshalled: "EWP 3 4 0 0"
        marshalled = marshal(&Envelope::new(3, 4, &vec!(), &vec!())).unwrap();
        assert_eq!(hex::encode(marshalled), format!("{}{}",hex::encode("EWP"),"00000003040000000000000000"));
    }

    #[test]
    fn test_marshal_successful() {
        struct Test {
            encoded: Envelope,
            message: String
        }
        let tests: Vec<Test> = vec!(
    		Test{
    			encoded: Envelope{
    				version:     13,
    				protocol:    0,
    				header:      "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
                message: format!("{}{}{}{}",hex::encode("EWP"),"0000000d00000000100000000e", hex::encode("this is a header".as_bytes()), hex::encode("this is a body".as_bytes()))
    		},
    		 Test{
    		 	encoded: Envelope{
    		 		version:     13,
    		 		protocol:    1,
    		 		header:      "testing".to_string().into_bytes(),
    		 		body:        "testing body".to_string().into_bytes(),
    		 	},
                 message: format!("{}{}{}{}",hex::encode("EWP"),"0000000d01000000070000000c", hex::encode("testing".as_bytes()), hex::encode("testing body".as_bytes()))
    		 },
    		 Test{
    		 	encoded: Envelope{
    		 		version:     1230329483,
    		 		protocol:    0,
    		 		header:      "test".to_string().into_bytes(),
    		 		body:        "test".to_string().into_bytes(),
    		 	},
                 message: format!("{}{}{}{}",hex::encode("EWP"),"4955568b000000000400000004", hex::encode("test".as_bytes()), hex::encode("test".as_bytes()))
    		 },
    	);

        for t in tests.iter() {
            let marshalled = marshal(&t.encoded).unwrap();
            println!("{}", t.message);
            println!("{}", hex::encode(&marshalled));
            assert!(hex::encode(marshalled) == t.message);
        }
    }
}
