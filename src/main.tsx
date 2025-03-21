import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import './main.css';
import Layout from './components/layout';
import { ThemeProvider } from './components/theme-provider';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
    <BrowserRouter>
      <Layout />
    </BrowserRouter>
  </ThemeProvider>,
);
