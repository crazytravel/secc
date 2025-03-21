import {
  Gauge,
  Globe,
  GlobeLock,
  Power,
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
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import { NavLink } from 'react-router';
import { Tabs, TabsList, TabsTrigger } from './ui/tabs';
import { Switch } from './ui/switch';

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

  return (
    <Sidebar className="">
      <SidebarHeader>
        <div className="space-y-2">
          <div className=" flex items-center space-x-4 rounded-md border p-4">
            <GlobeLock />
            <div className="flex-1 space-y-1">
              <p className="text-sm font-medium leading-none">Secc Connect</p>
            </div>
            <Switch />
          </div>
          <div className=" flex items-center space-x-4 rounded-md border p-4">
            <Tabs defaultValue="auto">
              <TabsList>
                <TabsTrigger value="auto">Auto Mode</TabsTrigger>
                <TabsTrigger value="global">Global Mode</TabsTrigger>
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
                      <SidebarMenuButton
                        className={isActive ? 'bg-gray-600' : ''}
                      >
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
