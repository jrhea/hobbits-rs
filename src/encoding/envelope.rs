
extern crate hex;
use std::fmt;

/// Envelope represents a parsed Hobbits message.
/// See examples of unparsed and parsed messages here: https://github.com/deltap2p/hobbits/blob/master/specs/protocol.md
#[derive(Clone, Hash, Default, PartialEq, Debug)]
pub struct Envelope {
    pub version: u32,
    pub protocol: u8,
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}

impl Envelope {
    #[inline]
    pub fn preamble() -> String {
        return "EWP".to_string();
    }
    pub fn new(version: u32, proto: u8, hdr: &[u8], bdy: &[u8]) -> Envelope {
        return Envelope {
            version: version,
            protocol: proto,
            header: hdr.to_vec(),
            body: bdy.to_vec()
        }
    }
}
impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {} {}",
            Envelope::preamble(), self.version, self.protocol, self.header.len(), self.body.len(), hex::encode(&self.header), hex::encode(&self.body))
	}
}
