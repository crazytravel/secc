import { AppSidebar } from '@/components/app-sidebar';
import { SidebarProvider } from '@/components/ui/sidebar';
import Tls from '@/pages/tls';
import { Route, Routes } from 'react-router';
import Dashboard from '../pages/dashboard';
import Rule from '../pages/rule';
import Server from '../pages/server';
import Setting from '../pages/setting';

export default function Layout() {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="w-full p-8">
        <Routes>
          <Route index element={<Dashboard />} />
          <Route path="/server" element={<Server />} />
          <Route path="/rule" element={<Rule />} />
          <Route path="/tls" element={<Tls />} />
          <Route path="/setting" element={<Setting />} />
        </Routes>
      </main>
    </SidebarProvider>
  );
}
