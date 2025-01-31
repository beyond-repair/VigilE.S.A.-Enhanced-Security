#[cfg(test)]
mod tests {
    use super::*;
    use aya::Bpf;

    #[tokio::test]
    async fn test_ebpf_policy() {
        let program = include_bytes!("../../../deployments/ebpf/firewall.bpf");
        let engine = EbpfPolicyEngine::load(program).unwrap();
        engine.apply_policy(Ipv4Addr::new(10,0,0,1), Ipv4Addr::new(10,0,0,2)).unwrap();
    }
}
