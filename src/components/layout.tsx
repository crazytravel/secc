import { AppSidebar } from '@/components/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import Tls from '@/pages/tls';
import { Route, Routes } from 'react-router';
import Dashboard from '../pages/dashboard';
import DirectRules from '../pages/direct-rules';
import ProxyRules from '../pages/proxy-rules';
import Servers from '../pages/servers';
import Setting from '../pages/setting';
import TrafficLogs from '@/pages/traffic-logs';
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export default function Layout() {
  useEffect(() => {
    const unLogListen = listen('refresh', () => {
      window.location.reload();
    });
    return () => {
      unLogListen.then((f) => f());
    };
  }, []);
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="w-full p-4">
        <Routes>
          <Route index element={<Dashboard />} />
          <Route path="/traffic-logs" element={<TrafficLogs />} />
          <Route path="/servers" element={<Servers />} />
          <Route path="/direct-rules" element={<DirectRules />} />
          <Route path="/proxy-rules" element={<ProxyRules />} />
          <Route path="/tls" element={<Tls />} />
          <Route path="/settings" element={<Setting />} />
        </Routes>
      </main>
    </SidebarProvider>
  );
}
