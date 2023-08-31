use digest::Digest;
use rand::distributions::Alphanumeric;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use sha2::Sha256;
use std::iter::Iterator;
use std::net::{IpAddr, Ipv4Addr};

// RandomVethName returns string "veth" with random prefix (hashed from entropy)
pub fn random_veth_name() -> String {
    let rng: SmallRng = SmallRng::from_entropy();
    let random_prefix: String = rng
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let mut hasher = Sha256::new();
    hasher.update(random_prefix.as_bytes());
    let hash_result = hasher.finalize();
    let hash_prefix = format!("{:x}", hash_result);

    format!("veth-{}", &hash_prefix[..8])
}

pub fn convert_to_ip(vec: Vec<u8>) -> Option<IpAddr> {
    if vec.len() == 4 {
        Some(IpAddr::V4(Ipv4Addr::new(vec[0], vec[1], vec[2], vec[3])))
    } else {
        None
    }
}
