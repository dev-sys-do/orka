use std::fmt::{Display, Formatter};

#[allow(dead_code)]
pub enum PluginsBin {
    OrkaCni,
    Bridge,
    HostLocal,
}

impl Display for PluginsBin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PluginsBin::OrkaCni => "orka-cni",
            PluginsBin::Bridge => "bridge",
            PluginsBin::HostLocal => "host-local",
        };
        write!(f, "{}", str)
    }
}
