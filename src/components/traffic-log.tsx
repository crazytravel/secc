import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { listen } from '@tauri-apps/api/event';
import { Logs } from 'lucide-react';
import { useEffect, useRef } from 'react';
import { Button } from './ui/button';

// Key for storing logs in local storage
const LOG_STORAGE_KEY = 'secc-traffic-logs';

export default function TrafficLog() {
  const logRef = useRef<HTMLElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const logsCache = useRef<string>(localStorage.getItem(LOG_STORAGE_KEY) || '');

  // Load cached logs on component mount
  useEffect(() => {
    if (logRef.current && logsCache.current) {
      logRef.current.innerText = logsCache.current;

      // Scroll to bottom initially if there are cached logs
      setTimeout(() => {
        if (containerRef.current) {
          containerRef.current.scrollTop = containerRef.current.scrollHeight;
        }
      }, 0);
    }
  }, []);

  useEffect(() => {
    const unLogListen = listen('secc-agent-log', (event) => {
      const newLog = event.payload as string;

      // Update cache ref
      logsCache.current += newLog;

      // Optional: Trim logs if they exceed max size
      if (logsCache.current.length > 500000) {
        logsCache.current = logsCache.current.substring(
          logsCache.current.length - 500000,
        );
      }

      // Update local storage without triggering renders
      localStorage.setItem(LOG_STORAGE_KEY, logsCache.current);

      // Direct DOM manipulation
      if (logRef.current) {
        logRef.current.innerText = logsCache.current;
      }

      console.log(containerRef.current?.scrollHeight);
      // Scroll to bottom
      if (containerRef.current) {
        containerRef.current.scrollTop = containerRef.current.scrollHeight;
      }
    });
    return () => {
      unLogListen.then((f) => f());
    };
  }, []);

  // Function to clear logs
  const clearLogs = () => {
    logsCache.current = '';
    localStorage.removeItem(LOG_STORAGE_KEY);
    if (logRef.current) {
      logRef.current.innerText = '';
    }
  };
  return (
    <div>
      <Card className="p-2">
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Logs /> <div>Traffic Log</div>
          </CardTitle>
        </CardHeader>
        <CardContent
          ref={containerRef}
          className="h-80 overflow-y-auto rounded-md border p-1"
        >
          <code ref={logRef} className="w-full text-[12px]"></code>
        </CardContent>
        <CardFooter>
          <Button onClick={clearLogs} variant="outline">
            Clear Logs
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
