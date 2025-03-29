import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Logs } from 'lucide-react';

export default function TrafficLog() {
  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Logs /> <div>Traffic Log</div>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[150px]">Time</TableHead>
                <TableHead className="w-[8px]">Log Type</TableHead>
                <TableHead className="w-[8px]">Nature</TableHead>
                <TableHead className="w-[80px]">TCP</TableHead>
                <TableHead>Message</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow>
                <TableCell className="font-medium">
                  2025-03-22 22:10:02
                </TableCell>
                <TableCell className="text-green-500">INFO</TableCell>
                <TableCell>PROXY</TableCell>
                <TableCell>TCP</TableCell>
                <TableCell>www.google.com:443</TableCell>
              </TableRow>
              <TableRow>
                <TableCell className="font-medium">
                  2025-03-22 22:09:10
                </TableCell>
                <TableCell className="text-green-500">INFO</TableCell>
                <TableCell>DIRECT</TableCell>
                <TableCell>TCP</TableCell>
                <TableCell>www.baidu.com:443</TableCell>
              </TableRow>
              <TableRow>
                <TableCell className="font-medium">
                  2025-03-22 22:09:08
                </TableCell>
                <TableCell className="text-red-500">ERROR</TableCell>
                <TableCell>PROXY</TableCell>
                <TableCell>UDP</TableCell>
                <TableCell>www.playstation.com:443</TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
