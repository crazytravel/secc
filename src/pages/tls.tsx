import Cert from '@/components/cert';
import CertKey from '@/components/cert-key';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Key, Shield } from 'lucide-react';
import { useEffect } from 'react';

function Tls() {
  useEffect(() => {}, []);

  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle>TLS Certification</CardTitle>
          <CardDescription>Status and mode change</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            <div className="rounded-md border space-y-4 p-4">
              <div className="flex items-center space-x-4">
                <Shield />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">
                    Certification
                  </p>
                </div>
              </div>
              <Cert />
            </div>
            <div className="rounded-md border space-y-4 p-4">
              <div className="flex items-center space-x-4">
                <Key />
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">
                    Certification Key
                  </p>
                </div>
              </div>
              <CertKey />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

export default Tls;
