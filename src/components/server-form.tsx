import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { toast } from 'sonner';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './ui/dialog';
import { Textarea } from './ui/textarea';
import { useState } from 'react';

const FormSchema = z.object({
  alias: z.string().optional(),
  host: z.string().nonempty(),
  quicPort: z.string().nonempty(),
  tcpPort: z.string().optional(),
  cert: z.string().nonempty(),
  certKey: z.string().nonempty(),
});

export default function ServerForm({
  host,
  callback,
}: {
  host?: string;
  callback: () => void;
}) {
  const [open, setOpen] = useState(false);

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    let res = await saveServerConfig(data);
    console.log(res);
    toast('success');
    callback();
    setOpen(false);
  }

  const loadServerConfig = async () => {
    let serverConfig = await invoke<ServerConfig>('get_server', { host });
    console.log('server config', serverConfig);
    if (serverConfig) {
      form.setValue('alias', serverConfig?.alias || '');
      form.setValue('host', serverConfig?.host || '');
      form.setValue('quicPort', serverConfig?.quic_port?.toString() || '');
      form.setValue('tcpPort', serverConfig?.tcp_port?.toString() || '');
      form.setValue('cert', serverConfig?.cert || '');
      form.setValue('certKey', serverConfig?.cert_key || '');
    }
  };

  const saveServerConfig = async (data: z.infer<typeof FormSchema>) => {
    if (host) {
      let res = await invoke<ServerConfig>('update_server', {
        server: {
          alias: data.alias,
          host: data.host,
          quic_port: parseInt(data.quicPort),
          tcp_port: data.tcpPort ? parseInt(data.tcpPort) : null,
          cert: data.cert,
          cert_key: data.certKey,
        },
      });
      console.log(res);
      return;
    }
    let res = await invoke<ServerConfig>('add_server', {
      server: {
        alias: data.alias,
        host: data.host,
        quic_port: parseInt(data.quicPort),
        tcp_port: data.tcpPort ? parseInt(data.tcpPort) : null,
        cert: data.cert,
        cert_key: data.certKey,
      },
    });
    console.log(res);
  };

  const handleOpenChange = async (open: boolean) => {
    form.clearErrors();
    form.reset();
    if (open && host) {
      loadServerConfig();
    }
    setOpen(open);
  };

  return (
    <div>
      <Dialog open={open} onOpenChange={handleOpenChange}>
        <DialogTrigger asChild>
          {host ? (
            <Button variant="ghost">Edit</Button>
          ) : (
            <Button>New Server</Button>
          )}
        </DialogTrigger>
        <DialogContent
          className="sm:max-w-[800px]"
          onInteractOutside={(e) => {
            e.preventDefault();
          }}
        >
          <DialogHeader>
            <DialogTitle>
              {host ? 'Edit Server' : 'Create New Server'}
            </DialogTitle>
            <DialogDescription>Set up new proxy server</DialogDescription>
          </DialogHeader>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <Form {...form}>
              <div className="p-4 rounded-md border space-y-4">
                <div className="space-x-4 flex items-center">
                  <FormField
                    control={form.control}
                    name="alias"
                    render={({ field }) => (
                      <FormItem className="flex-1">
                        <FormLabel>Alias</FormLabel>
                        <FormControl>
                          <Input placeholder="Alias" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="host"
                    render={({ field }) => (
                      <FormItem className="flex-2">
                        <FormLabel>
                          <span className="text-red-500">*</span>IP Address
                        </FormLabel>
                        <FormControl>
                          <Input
                            disabled={!!host}
                            placeholder="IP Address"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="quicPort"
                    render={({ field }) => (
                      <FormItem className="w-28">
                        <FormLabel>
                          <span className="text-red-500">*</span>Quic Port
                        </FormLabel>
                        <FormControl>
                          <Input placeholder="Quic Port" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="tcpPort"
                    render={({ field }) => (
                      <FormItem className="w-28">
                        <FormLabel>Tcp Port</FormLabel>
                        <FormControl>
                          <Input placeholder="Tcp Port" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
                <FormField
                  control={form.control}
                  name="cert"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>
                        <span className="text-red-500">*</span>Cert
                      </FormLabel>
                      <FormControl>
                        <Textarea
                          placeholder="Input your tls cert"
                          className="resize-none w-full"
                          {...field}
                          rows={5}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="certKey"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>
                        <span className="text-red-500">*</span>Cert Key
                      </FormLabel>
                      <FormControl>
                        <Textarea
                          placeholder="Input your tls cert key"
                          className="resize-none w-full"
                          {...field}
                          rows={5}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <DialogFooter>
                <Button type="submit">Submit</Button>
              </DialogFooter>
            </Form>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}
