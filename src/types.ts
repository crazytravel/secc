interface ServerConfig {
  host: string;
  quic_port?: number;
  tcp_port?: number;
  cert?: string;
  cert_key?: string;
  alias?: string;
}

interface ListenConfig {
  socksIp: string;
  socksPort: number;
  httpIp: string;
  httpPort: number;
}

interface Address {
  host: string;
  port: number;
}

interface ListenConfigOption {
  socks_config?: Address;
  http_config?: Address;
}
