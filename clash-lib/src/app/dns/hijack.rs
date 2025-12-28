use tracing::{trace, warn};

use crate::{
    app::dns::{ThreadSafeDNSResolver, exchange_with_resolver},
    proxy::datagram::UdpPacket,
};

pub async fn hijack_dns_packet<F>(
    resolver_dns: ThreadSafeDNSResolver,
    pkt: UdpPacket,
    packet_sink: F,
) where
    F: AsyncFnOnce(watfaq_netstack::UdpPacket) -> Result<(), std::io::Error>,
{
    match hickory_proto::op::Message::from_vec(&pkt.data) {
        Ok(msg) => {
            let send_response =
                async |msg: hickory_proto::op::Message, pkt: &UdpPacket| match msg
                    .to_vec()
                {
                    Ok(data) => {
                        if let Err(e) = packet_sink(
                            (
                                data,
                                pkt.dst_addr.clone().must_into_socket_addr(),
                                pkt.src_addr.clone().must_into_socket_addr(),
                            )
                                .into(),
                        )
                        .await
                        {
                            warn!("failed to send udp packet to netstack: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("failed to serialize dns response: {}", e);
                    }
                };

            trace!("hijack dns request: {:?}", msg);

            let mut resp =
                match exchange_with_resolver(&resolver_dns, &msg, true).await {
                    Ok(resp) => resp,
                    Err(e) => {
                        warn!("failed to exchange dns message: {}", e);
                        return;
                    }
                };

            // TODO: figure out where the message id got lost
            resp.set_id(msg.id());
            trace!("hijack dns response: {:?}", resp);

            send_response(resp, &pkt).await;
        }
        Err(e) => {
            warn!(
                "failed to parse dns packet: {}, putting it back to stack",
                e
            );
        }
    };
}
