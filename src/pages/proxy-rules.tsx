import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { zodResolver } from '@hookform/resolvers/zod';
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
import { Textarea } from '@/components/ui/textarea';
import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { Copy, Route } from 'lucide-react';
import { Input } from '@/components/ui/input';
import CombinedProxyRule from '@/components/combind-proxy-rule';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';

const FormSchema = z.object({
  communityRulesUrl: z.string(),
  proxyRules: z.string(),
});

const COMMUNITY_RULES_URL =
  'https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/proxy-list.txt';

function ProxyRules() {
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      communityRulesUrl: COMMUNITY_RULES_URL,
      proxyRules: '',
    },
  });

  async function onSubmit(data: z.infer<typeof FormSchema>) {
    await saveProxyRules(data);
    toast('success');
  }

  const loadProxyRules = async () => {
    let proxyRules = await invoke<string>('get_custom_proxy_rules');
    if (proxyRules) {
      form.setValue('proxyRules', proxyRules);
    }
  };

  const saveProxyRules = async (data: z.infer<typeof FormSchema>) => {
    await invoke('set_custom_proxy_rules', {
      proxyRules: data.proxyRules,
      url: data.communityRulesUrl,
    });
  };

  const copyText = () => {
    navigator.clipboard
      .writeText(COMMUNITY_RULES_URL)
      .then(() => toast('Copy success to clipboard!'))
      .catch((err) => toast('Copy failed', err));
  };

  useEffect(() => {
    loadProxyRules();
  }, []);
  return (
    <div>
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-1">
            <Route />
            <span>Proxy Rules</span>
          </CardTitle>
          <CardDescription className="flex items-center justify-between">
            <div>Custom Proxy Rules</div>
            <CombinedProxyRule />
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="p-4 rounded-md border">
            <Form {...form}>
              <form
                onSubmit={form.handleSubmit(onSubmit)}
                className="space-y-4"
              >
                <FormField
                  control={form.control}
                  name="communityRulesUrl"
                  render={({ field }) => (
                    <FormItem className="flex-1">
                      <FormLabel>
                        Community Proxy Rules Url
                        <TooltipProvider>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Copy
                                size="15"
                                className="cursor-pointer"
                                onClick={copyText}
                              />
                            </TooltipTrigger>
                            <TooltipContent>
                              <p>Copy community rule url</p>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>
                      </FormLabel>
                      <FormControl>
                        <Input
                          placeholder="Input community rules fetch url"
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="proxyRules"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Custom Proxy Rules</FormLabel>
                      <FormControl>
                        <Textarea
                          placeholder="Input your proxy domain list with new line"
                          className="resize-none w-full"
                          {...field}
                          rows={15}
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
        </CardContent>
      </Card>
    </div>
  );
}

export default ProxyRules;
