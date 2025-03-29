import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { ScrollArea } from '@radix-ui/react-scroll-area';
import { invoke } from '@tauri-apps/api/core';
import { forwardRef, useEffect, useImperativeHandle, useState } from 'react';
import { toast } from 'sonner';
import Confirm from './confirm';
import ServerForm from './server-form';

const ServerTable = forwardRef((_props, ref) => {
  const [servers, setServers] = useState<ServerConfig[]>([]);
  const loadData = async () => {
    let servers = await invoke<ServerConfig[]>('get_servers');
    setServers(servers);
  };

  const handleDelete = async (host: string) => {
    const res = await invoke('delete_server', { host });
    console.log(res);
    toast('success');
    loadData();
  };

  useImperativeHandle(ref, () => ({ loadData }));
  useEffect(() => {
    loadData();
  }, []);
  return (
    <ScrollArea className="overflow-auto h-[300px]">
      <Table>
        <TableHeader className="sticky top-0 bg-secondary rounded-md">
          <TableRow>
            <TableHead className="w-[10px]">No.</TableHead>
            <TableHead className="w-[15px]">Alias</TableHead>
            <TableHead>IP Address</TableHead>
            <TableHead className="w-[15px]">Quic Port</TableHead>
            <TableHead className="w-[15px]">Tcp Port</TableHead>
            <TableHead className="w-[20px]">Operation</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {servers?.map((server, index) => (
            <TableRow key={index}>
              <TableCell className="font-medium">{index + 1}</TableCell>
              <TableCell>{server.alias ? server.alias : '-'}</TableCell>
              <TableCell>{server.host}</TableCell>
              <TableCell>{server.quic_port}</TableCell>
              <TableCell>{server.tcp_port}</TableCell>
              <TableCell className="flex items-center space-x-2">
                <ServerForm host={server.host} callback={loadData} />
                <Confirm callback={() => handleDelete(server.host)}>
                  Delete
                </Confirm>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </ScrollArea>
  );
});

export default ServerTable;
