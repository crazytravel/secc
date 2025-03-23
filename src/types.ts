interface ServerConfig {
  host: string;
  port: number;
}

interface ListenConfig {
  socksIp: string;
  socksPort: number;
  httpIp: string;
  httpPort: number;
}

interface ListenConfigOption {
  socks_config?: ServerConfig;
  http_config?: ServerConfig;
}
