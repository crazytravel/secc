import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

import { invoke } from '@tauri-apps/api/core';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { useEffect } from 'react';

const FormSchema = z.object({
  ip: z.string().min(2, {
    message: 'IP must be at least 2 characters.',
  }),
  port: z.string().min(2, {
    message: 'Port must be at least 2 characters.',
  }),
});

export default function Server() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      ip: '',
      port: '443',
    },
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    toast('You submitted the following values:', {
      description: (
        <pre className="mt-2 w-[320px] rounded-md bg-slate-950 p-4">
          <code className="text-white w-full">
            {JSON.stringify(data, null, 2)}
          </code>
        </pre>
      ),
    });
    await saveServerConfig(data);
  }

  const loadServerConfig = async () => {
    let serverConfig = await invoke<ServerConfig>('get_server_config');
    console.log('server config', serverConfig);
    if (serverConfig) {
      form.setValue('ip', serverConfig.host);
      form.setValue('port', serverConfig.port.toString());
    }
  };

  const saveServerConfig = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_server_config', {
      serverConfig: {
        host: data.ip,
        port: parseInt(data.port),
      },
    });
  };

  useEffect(() => {
    loadServerConfig();
  }, []);

  return (
    <div className="space-y-8">
      <Card>
        <CardHeader>
          <CardTitle>Server Configuration</CardTitle>
          <CardDescription>
            Set proxy server address information
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="p-4 rounded-md border">
            <Form {...form}>
              <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="w-2/3  space-y-6"
              >
                <FormField
                  control={form.control}
                  name="ip"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>IP Address</FormLabel>
                      <FormControl>
                        <Input placeholder="IP Address" {...field} />
                      </FormControl>
                      <FormDescription>
                        This is your server IP address.
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="port"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Port</FormLabel>
                      <FormControl>
                        <Input placeholder="Port" {...field} />
                      </FormControl>
                      <FormDescription>
                        This is your server port.
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <Button type="submit">Submit</Button>
              </form>
            </Form>
          </div>
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
