import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from '@/components/ui/form';
import { Textarea } from '@/components/ui/textarea';
import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';
import { toast } from 'sonner';

const FormSchema = z.object({
  proxyRules: z.string(),
});

function ProxyRule() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    await saveProxyRules(data);
    toast('success');
  }

  const loadProxyRules = async () => {
    let proxyRules = await invoke<string>('get_proxy_rules');
    if (proxyRules) {
      form.setValue('proxyRules', proxyRules);
    }
  };

  const saveProxyRules = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_proxy_rules', {
      proxyRules: data.proxyRules,
    });
  };

  useEffect(() => {
    loadProxyRules();
  }, []);

  return (
    <div>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <FormField
            control={form.control}
            name="proxyRules"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Textarea
                    placeholder="Input your proxy domain list with new line"
                    className="resize-none w-full"
                    {...field}
                    rows={10}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Submit</Button>
        </form>
      </Form>
    </div>
  );
}

export default ProxyRule;
