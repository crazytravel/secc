import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import './main.css';
import Layout from './components/layout';
import { ThemeProvider } from './components/theme-provider';
import { Toaster } from './components/ui/sonner';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
    <BrowserRouter>
      <Layout />
      <Toaster />
    </BrowserRouter>
  </ThemeProvider>,
);
