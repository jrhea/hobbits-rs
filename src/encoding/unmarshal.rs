
pub use crate::encoding::envelope::Envelope;
pub use crate::encoding::marshal;
pub use crate::encoding::EwpError;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::TryFrom;

pub fn unmarshal(msg: &[u8]) -> Result<Envelope,EwpError> {
    let mut index = Envelope::preamble().len();
    let preamble = String::from_utf8(msg[0..index].to_vec()).unwrap();
    let mut tmp = &msg[index..(index+4)];
    let mut rdr = Cursor::new(tmp.to_vec());
    // version
    let version = rdr.read_u32::<BigEndian>().unwrap();
    index = index + 4;
    tmp = &msg[index..(index+1)];
    rdr = Cursor::new(tmp.to_vec());
    // protocol
    let protocol = rdr.read_u8().unwrap();;
    index = index + 1;
    tmp = &msg[index..(index+4)];
    rdr = Cursor::new(tmp.to_vec());
    // header length
    let hdr_len = usize::try_from(rdr.read_u32::<BigEndian>().unwrap()).unwrap();
    index = index + 4;
    tmp = &msg[index..(index+4)];
    rdr = Cursor::new(tmp.to_vec());
    // body length
    let bdy_len = usize::try_from(rdr.read_u32::<BigEndian>().unwrap()).unwrap();
    index = index + 4;
    // header
    let hdr = &msg[index..(index+hdr_len)];
    index = index + hdr_len;
    // body
    let bdy = &msg[index..(index+bdy_len)];

    Ok( Envelope {
        version: version,
        protocol: protocol,
        header: hdr.to_owned(),
        body: bdy.to_owned()
    })
}


#[cfg(test)]
mod tests {
    use super::{Envelope, unmarshal, marshal};

    #[test]
    fn test_unmarshal_successful() {
        struct Test {
            message: String, // WARN! we're loading this from utf-8 strings, so don't use non-ascii string content
            output: Envelope
        } 
        let tests: Vec<Test> = vec!(
    		Test {
    			message: format!("{}{}{}{}",hex::encode("EWP".as_bytes()),"0000000d00000000100000000e", hex::encode("this is a header".as_bytes()), hex::encode("this is a body".as_bytes())),
    			output: Envelope {
    				version:     13,
    				protocol:    0,
    				header:      "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
    		},
    		Test {
    			message: format!("{}{}{}{}",hex::encode("EWP".as_bytes()),"0000000d01000000070000000c", hex::encode("testing".as_bytes()), hex::encode("testing body".as_bytes())),
    			output: Envelope {
    				version:     13,
    				protocol:    1,
    				header:      "testing".to_string().into_bytes(),
    				body:        "testing body".to_string().into_bytes(),
    			},
    		},
    		Test {
    			message: format!("{}{}{}{}",hex::encode("EWP".as_bytes()),"4955568b000000000400000004", hex::encode("test".as_bytes()), hex::encode("test".as_bytes())),
    			output: Envelope {
    				version:     1230329483,
    				protocol:    0,
    				header:      "test".to_string().into_bytes(),
    				body:        "test".to_string().into_bytes(),
    			},
    		},
    	);

        for t in tests.iter() {
            let unmarshalled = unmarshal(&hex::decode(t.message.clone()).unwrap());
            match unmarshalled {
                Ok(msg) => {
                    println!("expected {}", t.message);
                    println!("recieved {}", msg);
                    assert!(msg.to_string() == t.output.to_string());
                }
                Err(err) => {
                    println!("error {}", err);
                    assert!(false);
                }
            }
            
        }
    }

    #[test]
    fn test_roundtrip() {
        struct Test {
            message: Envelope,
        }
        let tests: Vec<Test> = vec!(
    		Test {
    			message: Envelope {
    				version:     13,
    				protocol:    0,
    				header:      "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
    		}
        );
        for t in tests.iter() {
            let marshalled = marshal(&t.message);
            match marshalled {
                Ok(m_msg) => {
                    let unmarshalled = unmarshal(&m_msg);
                    match unmarshalled {
                        Ok(u_msg) => {
                            println!("expected {}", t.message);
                            println!("recieved {}", u_msg);
                            assert!(u_msg.to_string() == t.message.to_string());
                        }
                        Err(err) => {
                            println!("error {}", err);
                            assert!(false);
                        }
                    }
                }
                Err(err) => {
                    println!("error {}", err);
                    assert!(false);
                }
            }
        }
    }
}
