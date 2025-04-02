import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

import ServerForm from '@/components/server-form';
import ServerTable from '@/components/server-table';
import { useRef } from 'react';
import { Server } from 'lucide-react';

export default function Servers() {
  const serverTableRef = useRef<{ loadData: () => void } | null>(null);

  const handleCreateSucceed = () => {
    serverTableRef.current?.loadData();
  };

  return (
    <div className="space-y-8">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Server /> <div>Server Configuration</div>
          </CardTitle>
          <CardDescription>
            Set proxy server address information
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-2 h-[350px] flex flex-col">
          <div className="flex-1">
            <ServerTable ref={serverTableRef} />
          </div>
          <ServerForm callback={handleCreateSucceed} />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Server Installation</CardTitle>
          <CardDescription>Install secc to your server</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="p-4 rounded-md border">TODO</div>
        </CardContent>
      </Card>
    </div>
  );
}
