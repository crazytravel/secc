import {
  Gauge,
  GlobeLock,
  Route,
  Server,
  Settings,
  ShieldCheck,
} from 'lucide-react';

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
import { NavLink } from 'react-router';
import { Switch } from './ui/switch';
import { Tabs, TabsList, TabsTrigger } from './ui/tabs';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

export function AppSidebar() {
  // Menu items.
  const items = [
    {
      title: 'Dashboard',
      url: '/',
      icon: Gauge,
    },
    {
      title: 'Server',
      url: '/server',
      icon: Server,
    },
    {
      title: 'Bypass Rule',
      url: '/rule',
      icon: Route,
    },
    {
      title: 'TLS Certification',
      url: '/tls',
      icon: ShieldCheck,
    },
    {
      title: 'Settings',
      url: '/setting',
      icon: Settings,
    },
  ];

  const [accessMode, setAccessMode] = useState('auto');
  const [bindMode, setBindMode] = useState('socks');
  const [protocolMode, setProtocolMode] = useState('quic');

  const handleSeccSwitch = async (checked: boolean) => {
    console.log('checked', checked);
    if (checked) {
      await invoke('open_secc', {});
      return;
    }
    await invoke('close_secc', {});
  };

  const handleAccessModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_access_mode', { accessMode: value });
  };

  const handleBindModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_bind_mode', { bindMode: value });
  };

  const handleProtocolModeSwitch = async (value: string) => {
    console.log('value', value);
    await invoke('switch_protocol_mode', { protocolMode: value });
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

  useEffect(() => {
    getAccessMode();
    getBindMode();
    getProtocolMode();
  }, []);

  return (
    <Sidebar>
      <SidebarHeader>
        <div className="space-y-2">
          <div className=" flex items-center space-x-2 rounded-md border p-4">
            <GlobeLock />
            <div className="flex-1 space-y-1">
              <p className="text-sm font-medium leading-none">Secc Connect</p>
            </div>
            <Switch
              defaultChecked
              className="data-[state=checked]:bg-green-500"
              onCheckedChange={handleSeccSwitch}
            />
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs
              defaultValue={accessMode}
              onValueChange={handleAccessModeSwitch}
            >
              <TabsList>
                <TabsTrigger value="auto">Auto Mode</TabsTrigger>
                <TabsTrigger value="proxy">Global Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs defaultValue={bindMode} onValueChange={handleBindModeSwitch}>
              <TabsList>
                <TabsTrigger value="socks">Socks Mode</TabsTrigger>
                <TabsTrigger value="http">Http Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
          <div className="flex items-center space-x-4 rounded-md border p-4">
            <Tabs
              defaultValue={protocolMode}
              onValueChange={handleProtocolModeSwitch}
            >
              <TabsList>
                <TabsTrigger value="quic">Quic Mode</TabsTrigger>
                <TabsTrigger value="tcp-udp">TcpUdp Mode</TabsTrigger>
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
