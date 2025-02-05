use libp2p::{
    core::transport::upgrade,
    dns::DnsConfig,
    identity,
    noise::{Keypair, NoiseConfig, X25519Spec},
    relay::{self, RelayConfig},
    swarm::SwarmBuilder,
    tcp::TcpConfig,
    yamux::YamuxConfig,
    Multiaddr, PeerId, Swarm, Transport,
};
use std::error::Error;
use tokio::runtime::Runtime;

pub fn start_relay_server() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let transport = DnsConfig::system(TcpConfig::new().nodelay(true))?
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(Keypair::<X25519Spec>::new().into_authentic(&local_key)?))
        .multiplex(YamuxConfig::default())
        .boxed();

    let relay_config = RelayConfig::default();
    let behaviour = relay::Behaviour::new(local_peer_id, relay_config);

    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    let rt = Runtime::new()?;
    rt.block_on(async {
        loop {
            match swarm.next().await {
                _ => {}
            }
        }
    })?;

    Ok(())
}
