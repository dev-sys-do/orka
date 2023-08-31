use std::fmt;

pub enum NetworkConfigReference {
    Name,
    Type,
    Bridge,
    IsGateway,
    IsDefaultGateway,
    ForceAddress,
    IpMasq,
    Mtu,
    HairpinMode,
    Ipam,
    PromiscMode,
    Vlan,
    PreserveDefaultVlan,
    VlanTrunk,
    Enabledad,
    Macspoofchk,
}

impl fmt::Display for NetworkConfigReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkConfigReference::Name => write!(f, "name"),
            NetworkConfigReference::Type => write!(f, "type"),
            NetworkConfigReference::Bridge => write!(f, "bridge"),
            NetworkConfigReference::IsGateway => write!(f, "isGateway"),
            NetworkConfigReference::IsDefaultGateway => write!(f, "isDefaultGateway"),
            NetworkConfigReference::ForceAddress => write!(f, "forceAddress"),
            NetworkConfigReference::IpMasq => write!(f, "ipMasq"),
            NetworkConfigReference::Mtu => write!(f, "mtu"),
            NetworkConfigReference::HairpinMode => write!(f, "hairpinMode"),
            NetworkConfigReference::Ipam => write!(f, "ipam"),
            NetworkConfigReference::PromiscMode => write!(f, "promiscMode"),
            NetworkConfigReference::Vlan => write!(f, "vlan"),
            NetworkConfigReference::PreserveDefaultVlan => write!(f, "preserveDefaultVlan"),
            NetworkConfigReference::VlanTrunk => write!(f, "vlanTrunk"),
            NetworkConfigReference::Enabledad => write!(f, "enabledad"),
            NetworkConfigReference::Macspoofchk => write!(f, "macspoofchk"),
        }
    }
}
