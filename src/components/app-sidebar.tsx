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

  const [accessMode, setAccessMode] = useState('Auto');
  const [bindMode, setBindMode] = useState('Socks');

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

  useEffect(() => {
    getAccessMode();
    getBindMode();
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
          <div className=" flex items-center space-x-4 rounded-md border p-4">
            <Tabs
              defaultValue={accessMode}
              onValueChange={handleAccessModeSwitch}
            >
              <TabsList>
                <TabsTrigger value="Auto">Auto Mode</TabsTrigger>
                <TabsTrigger value="Proxy">Proxy Mode</TabsTrigger>
              </TabsList>
            </Tabs>
          </div>
          <div className=" flex items-center space-x-4 rounded-md border p-4">
            <Tabs defaultValue={bindMode} onValueChange={handleBindModeSwitch}>
              <TabsList>
                <TabsTrigger value="Socks">Socks Mode</TabsTrigger>
                <TabsTrigger value="Http">Http Mode</TabsTrigger>
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
