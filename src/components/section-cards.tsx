import { ArrowDown, ArrowUp } from 'lucide-react';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

export function SectionCards() {
  return (
    <div className="w-full flex flex-row gap-8">
      <Card className="flex-1">
        <CardHeader>
          <CardTitle className="flex items-center text-green-500">
            <ArrowUp />
            <div>Up</div>
          </CardTitle>
        </CardHeader>
        <CardContent className="text-2xl">1,234 kb/s</CardContent>
      </Card>
      <Card className="flex-1">
        <CardHeader>
          <CardTitle className="flex items-center text-blue-500">
            <ArrowDown />
            <div>Down</div>
          </CardTitle>
        </CardHeader>
        <CardContent className="text-2xl">1,250.00 kb/s</CardContent>
      </Card>
    </div>
  );
}
