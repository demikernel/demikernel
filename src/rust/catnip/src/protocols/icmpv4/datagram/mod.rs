mod echo;
mod header;

pub use echo::{Icmpv4Echo, Icmpv4EchoMut, Icmpv4EchoOp};
pub use header::{
    Icmpv4Header, Icmpv4HeaderMut, Icmpv4Type, ICMPV4_HEADER_SIZE,
};

use crate::{prelude::*, protocols::ipv4};
use std::convert::TryFrom;

pub struct Icmpv4Datagram<'a>(ipv4::Datagram<'a>);

impl<'a> Icmpv4Datagram<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        Ok(Icmpv4Datagram::try_from(ipv4::Datagram::from_bytes(
            bytes,
        )?)?)
    }

    pub fn header(&self) -> Icmpv4Header<'_> {
        Icmpv4Header::new(&self.0.payload()[..ICMPV4_HEADER_SIZE])
    }

    pub fn ipv4(&self) -> &ipv4::Datagram<'a> {
        &self.0
    }

    pub fn payload(&self) -> &[u8] {
        &self.0.payload()[ICMPV4_HEADER_SIZE..]
    }
}

impl<'a> TryFrom<ipv4::Datagram<'a>> for Icmpv4Datagram<'a> {
    type Error = Fail;

    fn try_from(ipv4_datagram: ipv4::Datagram<'a>) -> Result<Self> {
        assert_eq!(ipv4_datagram.header().protocol()?, ipv4::Protocol::Icmpv4);
        if ipv4_datagram.payload().len() < ICMPV4_HEADER_SIZE {
            return Err(Fail::Malformed {
                details: "ICMPv4 datagram isn't large enough to contain a \
                          complete header",
            });
        }

        let icmpv4 = Icmpv4Datagram(ipv4_datagram);
        let mut checksum = ipv4::checksum::Hasher::new();
        let payload = icmpv4.ipv4().payload();
        checksum.write(&payload[..2]);
        checksum.write(&payload[4..]);
        if checksum.finish() != icmpv4.header().checksum() {
            return Err(Fail::Malformed {
                details: "ICMPv4 checksum mismatch",
            });
        }

        Ok(icmpv4)
    }
}

pub struct Icmpv4DatagramMut<'a>(ipv4::DatagramMut<'a>);

impl<'a> Icmpv4DatagramMut<'a> {
    pub fn new_bytes() -> Vec<u8> {
        ipv4::DatagramMut::new_bytes(ICMPV4_HEADER_SIZE)
    }

    pub fn from_bytes(bytes: &'a mut [u8]) -> Result<Self> {
        Ok(Icmpv4DatagramMut(ipv4::DatagramMut::from_bytes(bytes)?))
    }

    pub fn header(&mut self) -> Icmpv4HeaderMut<'_> {
        Icmpv4HeaderMut::new(&mut self.0.payload()[..ICMPV4_HEADER_SIZE])
    }

    pub fn ipv4(&mut self) -> &mut ipv4::DatagramMut<'a> {
        &mut self.0
    }

    pub fn payload(&mut self) -> &mut [u8] {
        &mut self.0.payload()[ICMPV4_HEADER_SIZE..]
    }

    #[allow(dead_code)]
    pub fn unmut(self) -> Result<Icmpv4Datagram<'a>> {
        Ok(Icmpv4Datagram::try_from(self.0.unmut()?)?)
    }

    pub fn seal(mut self) -> Result<Icmpv4Datagram<'a>> {
        trace!("Icmp4DatagramMut::seal()");
        self.ipv4().header().protocol(ipv4::Protocol::Icmpv4);
        let mut checksum = ipv4::checksum::Hasher::new();
        checksum.write(self.0.payload());
        self.header().checksum(checksum.finish());
        Ok(Icmpv4Datagram::try_from(self.0.seal()?)?)
    }
}

impl<'a> From<ipv4::DatagramMut<'a>> for Icmpv4DatagramMut<'a> {
    fn from(ipv4_datagram: ipv4::DatagramMut<'a>) -> Self {
        Icmpv4DatagramMut(ipv4_datagram)
    }
}
