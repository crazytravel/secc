import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { ScrollArea } from '@/components/ui/scroll-area';
import { listen } from '@tauri-apps/api/event';
import { Loader, ScrollText } from 'lucide-react';
import { useEffect, useRef } from 'react';

// Key for storing logs in local storage
const LOG_STORAGE_KEY = 'secc-traffic-logs';

export default function TrafficLogs() {
  const logRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const logsCache = useRef<string>(localStorage.getItem(LOG_STORAGE_KEY) || '');

  useEffect(() => {
    if (logRef.current) {
      logRef.current.innerHTML = logsCache.current;
    }
    const unLogListen = listen('secc-agent-log', (event) => {
      let newLog = event.payload as string;
      newLog = newLog.replace(
        'ERROR',
        '<span class="text-red-500">ERROR</span>',
      );
      newLog = newLog.replace(
        'INFO',
        '<span class="text-green-500">INFO</span>',
      );
      newLog = newLog.replace(
        'PROXY',
        '<span class="text-orange-500">PROXY</span>',
      );
      newLog = newLog.replace(
        'DIRECT',
        '<span class="text-blue-500">DIRECT</span>',
      );
      // Update cache ref
      logsCache.current += `<div>${newLog}</div>`;

      // Optional: Trim logs if they exceed max size
      if (logsCache.current.length > 50000) {
        logsCache.current = logsCache.current.substring(
          logsCache.current.length - 50000,
        );
      }

      // Update local storage without triggering renders
      localStorage.setItem(LOG_STORAGE_KEY, logsCache.current);
      if (logRef.current) {
        logRef.current.innerHTML = logsCache.current;
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
      logRef.current.textContent = '';
    }
  };
  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <ScrollText /> <div>Traffic Logs</div>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <ScrollArea className="h-130 w-full rounded-md border">
            <div ref={containerRef} className="h-full overflow-auto">
              <div ref={logRef} className="p-2 text-sm">
                <Loader className="animate-spin" />
              </div>
            </div>
          </ScrollArea>
        </CardContent>
        <CardFooter>
          <Button onClick={clearLogs} variant="default">
            Clear Logs
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
