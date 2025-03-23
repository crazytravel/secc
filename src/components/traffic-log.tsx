import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { listen } from '@tauri-apps/api/event';
import { Logs } from 'lucide-react';
import { useEffect, useRef } from 'react';

export default function TrafficLog() {
  const logRef = useRef<HTMLElement | null>(null);
  useEffect(() => {
    const unLogListen = listen('secc-agent-log', (event) => {
      const log = event.payload;
      console.log(log);
      if (logRef.current) {
        logRef.current.innerText += log;
      }
    });
    return () => {
      unLogListen.then((f) => f());
    };
  }, []);

  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Logs /> <div>Traffic Log</div>
          </CardTitle>
        </CardHeader>
        <CardContent className="h-96 overflow-y-auto">
          <code ref={logRef} className="text-white"></code>
        </CardContent>
      </Card>
    </div>
  );
}
