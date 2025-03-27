import DirectRule from '@/components/direct-rule';
import CustomProxyRule from '@/components/custom-proxy-rule';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { ArrowRightLeft, Waypoints } from 'lucide-react';
import CombinedProxyRule from '@/components/combind-proxy-rule';

function Rule() {
  const reload = () => {
    window.location.reload();
  };
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
                  <p className="text-sm font-medium leading-none">
                    Custom Proxy Rule
                  </p>
                </div>
              </div>
              <CustomProxyRule callback={reload} />
            </div>
            <div className="rounded-md border space-y-4 p-4">
              <div className="flex items-center space-x-4">
                <Waypoints />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">
                    Combined Proxy Rule
                  </p>
                </div>
              </div>
              <CombinedProxyRule />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

export default Rule;
