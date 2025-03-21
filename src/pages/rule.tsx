import DirectRule from '@/components/direct-rule';
import ProxyRule from '@/components/proxy-rule';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Switch } from '@/components/ui/switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { GlobeLock, Waypoints, Power, ArrowRightLeft } from 'lucide-react';
import { useEffect } from 'react';

function Rule() {
  useEffect(() => {}, []);

  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle>Bypass Rules</CardTitle>
          <CardDescription>Status and mode change</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            <div className="rounded-md border space-y-4 p-4">
              <div className="flex items-center space-x-4">
                <ArrowRightLeft />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">
                    Direct Rule
                  </p>
                </div>
              </div>
              <DirectRule />
            </div>
            <div className="rounded-md border space-y-4 p-4">
              <div className="flex items-center space-x-4">
                <Waypoints />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">Proxy Rule</p>
                </div>
              </div>
              <ProxyRule />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

export default Rule;
