import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from '@/components/ui/form';
import { Textarea } from '@/components/ui/textarea';
import { toast } from 'sonner';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const FormSchema = z.object({
  cert: z.string(),
});

function Cert() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    await saveCert(data);
    toast('success');
  }

  const loadCert = async () => {
    let cert = await invoke<string>('get_cert');
    if (cert) {
      form.setValue('cert', cert);
    }
  };

  const saveCert = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_cert', {
      cert: data.cert,
    });
  };

  useEffect(() => {
    loadCert();
  }, []);

  return (
    <div>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <FormField
            control={form.control}
            name="cert"
            render={({ field }) => (
              <FormItem>
                <FormMessage />
                <FormControl>
                  <Textarea
                    placeholder="Input your tls cert"
                    className="resize-none w-full"
                    {...field}
                    rows={5}
                  />
                </FormControl>
              </FormItem>
            )}
          />
        </form>
      </Form>
    </div>
  );
}

export default Cert;
