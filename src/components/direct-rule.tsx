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
import { toast } from 'sonner';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

const FormSchema = z.object({
  directRules: z.string(),
});

function DirectRule() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    await saveDirectRules(data);
    toast('success');
  }

  const loadDirectRules = async () => {
    let directRules = await invoke<string>('get_direct_rules');
    if (directRules) {
      form.setValue('directRules', directRules);
    }
  };

  const saveDirectRules = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_direct_rules', {
      directRules: data.directRules,
    });
  };

  useEffect(() => {
    loadDirectRules();
  }, []);

  return (
    <div>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          <FormField
            control={form.control}
            name="directRules"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Textarea
                    placeholder="Input your direct domain list with new line"
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

export default DirectRule;
