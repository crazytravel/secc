import { Gauge, Route, ScrollText, Server, Settings, Zap } from 'lucide-react';

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { NavLink } from 'react-router';
import { Switch } from './ui/switch';
import { Tabs, TabsList, TabsTrigger } from './ui/tabs';
import Logo from '@/assets/icon.png';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'sonner';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from './ui/select';

export function AppSidebar() {
  // Menu items.
  const items = [
    {
      title: 'Dashboard',
      url: '/',
      icon: Gauge,
    },
    {
      title: 'Traffic Logs',
      url: '/traffic-logs',
      icon: ScrollText,
    },
    {
      title: 'Servers',
      url: '/servers',
      icon: Server,
    },
    {
      title: 'Direct Rules',
      url: '/direct-rules',
      icon: Zap,
    },
    {
      title: 'Proxy Rules',
      url: '/proxy-rules',
      icon: Route,
    },
    {
      title: 'Settings',
      url: '/settings',
      icon: Settings,
    },
  ];

  const [servers, setServers] = useState<ServerConfig[]>([]);
  const [connected, setConnected] = useState(false);
  const [activeServer, setActiveServer] = useState('');
  const [accessMode, setAccessMode] = useState('auto');
  const [bindMode, setBindMode] = useState('socks');
  const [protocolMode, setProtocolMode] = useState('tcp');

  const handleSeccSwitch = async (checked: boolean) => {
    console.log('checked', checked);
    if (checked && activeServer === '') {
      toast('Please select a server first');
      setConnected(false);
      return;
    }
    if (checked && activeServer !== '') {
      await invoke('open_secc', {});
      setConnected(true);
      return;
    }
    if (!checked && activeServer !== '') {
      await invoke('close_secc', {});
      setConnected(false);
    }
  };

  const handleAccessModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_access_mode', { accessMode: value });
    setAccessMode(value);
  };

  const handleBindModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_bind_mode', { bindMode: value });
    setBindMode(value);
  };

  const handleProtocolModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_protocol_mode', { protocolMode: value });
    setProtocolMode(value);
  };

  const getAccessMode = async () => {
    let accessMode = await invoke<string>('get_access_mode');
    if (accessMode) {
      setAccessMode(accessMode);
    }
  };

  const getBindMode = async () => {
    let bindMode = await invoke<string>('get_bind_mode');
    if (bindMode) {
      setBindMode(bindMode);
    }
  };

  const getProtocolMode = async () => {
    let protocolMode = await invoke<string>('get_protocol_mode');
    if (protocolMode) {
      setProtocolMode(protocolMode);
    }
  };

  const getActiveServer = async () => {
    let server = await invoke<string>('get_active_server');
    console.log('active server', server);
    if (server) {
      setActiveServer(server);
      setConnected(true);
    }
  };

  const getServers = async () => {
    let servers = await invoke<ServerConfig[]>('get_servers');
    if (servers) {
      setServers(servers);
    }
  };

  const handleServerSelect = async (value: string) => {
    console.log('select:', value);
    setActiveServer(value);
    setConnected(true);
    await invoke('active_server', { host: value });
    await invoke('open_secc', {});
  };

  useEffect(() => {
    getAccessMode();
    getBindMode();
    getProtocolMode();
    getServers();
    getActiveServer();

    const unListen = listen('active_server_disable', () => {
      setActiveServer('');
    });
    const unRefreshListen = listen('refresh_servers', () => {
      getServers();
    });
    return () => {
      unListen.then((f) => f());
      unRefreshListen.then((f) => f());
    };
  }, []);

  return (
    <Sidebar>
      <SidebarHeader>
        <div className="flex items-center space-x-2 p-2">
          <img src={Logo} className="w-12 h-12" />
          <div className="flex flex-col justify-center">
            <h1 className="text-md font-bold">SECC</h1>
            <h2 className="text-sm">Secure Your Connection</h2>
          </div>
        </div>
        <div className="space-y-2">
          <div className=" flex items-center space-x-2 rounded-md border p-4">
            <Select onValueChange={handleServerSelect} value={activeServer}>
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="Select a server" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Servers</SelectLabel>
                  {servers?.map((server, index) => (
                    <SelectItem key={index} value={server.host}>
                      {server.alias ? server.alias : server.host}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>
            <Switch
              checked={connected}
              className="data-[state=checked]:bg-green-500"
              onCheckedChange={handleSeccSwitch}
            />
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs value={accessMode} onValueChange={handleAccessModeSwitch}>
              <TabsList>
                <TabsTrigger value="auto">Auto Mode</TabsTrigger>
                <TabsTrigger value="proxy">Global Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs value={bindMode} onValueChange={handleBindModeSwitch}>
              <TabsList>
                <TabsTrigger value="socks">Socks Mode</TabsTrigger>
                <TabsTrigger value="http">Http Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs value={protocolMode} onValueChange={handleProtocolModeSwitch}>
              <TabsList>
                <TabsTrigger value="tcp">Tcp Mode</TabsTrigger>
                <TabsTrigger value="quic">Quic Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
        </div>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((project) => (
                <NavLink to={project.url} key={project.title}>
                  {({ isActive }) => (
                    <SidebarMenuItem>
                      <SidebarMenuButton isActive={isActive}>
                        <project.icon />
                        <span>{project.title}</span>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  )}
                </NavLink>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
