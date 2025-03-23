import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
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
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const FormSchema = z.object({
  socksIp: z.string().min(2, {
    message: 'socksIp must be at least 2 characters.',
  }),
  socksPort: z.string().min(2, {
    message: 'socksPort must be at least 2 characters.',
  }),
  httpIp: z.string().min(2, {
    message: 'httpIp must be at least 2 characters.',
  }),
  httpPort: z.string().min(2, {
    message: 'httpPort must be at least 2 characters.',
  }),
});

function Setting() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      socksIp: '127.0.0.1',
      socksPort: '1080',
      httpIp: '127.0.0.1',
      httpPort: '1081',
    },
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    toast('You submitted the following values:', {
      description: (
        <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
          <code className="text-white">{JSON.stringify(data, null, 2)}</code>
        </pre>
      ),
    });
    await saveListenConfig(data);
  }

  const loadListenConfig = async () => {
    let listenConfig = await invoke<ListenConfigOption>('get_listen_config');
    if (listenConfig) {
      form.setValue('socksIp', listenConfig.socks_config?.host || '');
      form.setValue(
        'socksPort',
        listenConfig.socks_config?.port.toString() || '',
      );
      form.setValue('httpIp', listenConfig.http_config?.host || '');
      form.setValue(
        'httpPort',
        listenConfig.http_config?.port.toString() || '',
      );
    }
  };

  const saveListenConfig = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_listen_config', {
      listenConfig: {
        socks_ip: data.socksIp,
        socks_port: parseInt(data.socksPort),
        http_ip: data.httpIp,
        http_port: parseInt(data.httpPort),
      },
    });
  };

  useEffect(() => {
    loadListenConfig();
  }, []);

  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle>Settings</CardTitle>
          <CardDescription>Set listening address information</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="p-4 rounded-md border">
            <Form {...form}>
              <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="w-2/3 space-y-6"
              >
                <FormField
                  control={form.control}
                  name="socksIp"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Socks5 IP Address</FormLabel>
                      <FormControl>
                        <Input placeholder="IP Address" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="socksPort"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Socks5 Port</FormLabel>
                      <FormControl>
                        <Input placeholder="Port" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="httpIp"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Http IP Address</FormLabel>
                      <FormControl>
                        <Input placeholder="IP Address" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="httpPort"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Http Port</FormLabel>
                      <FormControl>
                        <Input placeholder="Port" {...field} />
                      </FormControl>
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
    </div>
  );
}

export default Setting;
