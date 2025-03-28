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
import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';
import { toast } from 'sonner';

const FormSchema = z.object({
  certKey: z.string(),
});

function CertKey() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    await saveCertKey(data);
    toast('success');
  }

  const loadCertKey = async () => {
    let certKey = await invoke<string>('get_cert');
    if (certKey) {
      form.setValue('certKey', certKey);
    }
  };

  const saveCertKey = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_cert', {
      certKey: data.certKey,
    });
  };

  useEffect(() => {
    loadCertKey();
  }, []);

  return (
    <div>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <FormField
            control={form.control}
            name="certKey"
            render={({ field }) => (
              <FormItem>
                <FormMessage />
                <FormControl>
                  <Textarea
                    placeholder="Input your tls cert key"
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

export default CertKey;
