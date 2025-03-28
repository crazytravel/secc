import { invoke } from '@tauri-apps/api/core';
import { Loader } from 'lucide-react';
import { useRef } from 'react';
import { Button } from './ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './ui/dialog';
import { ScrollArea } from './ui/scroll-area';

function CombinedProxyRule() {
  const ruleRef = useRef<HTMLDivElement>(null);
  const loadProxyRules = async () => {
    let proxyRules = await invoke<string>('get_combined_proxy_rules');
    if (proxyRules) {
      if (ruleRef.current) {
        ruleRef.current.innerText = proxyRules;
      }
    }
  };

  const handleOpenChange = async (open: boolean) => {
    if (open) {
      await loadProxyRules();
    }
  };

  return (
    <Dialog onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        <Button variant="outline">Combined Rules</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[680px]">
        <DialogHeader>
          <DialogTitle>Combined Proxy Rules</DialogTitle>
          <DialogDescription>
            Combine custom proxy rules and community proxy rules
          </DialogDescription>
        </DialogHeader>
        <ScrollArea className="h-70 w-full rounded-md border">
          <div className="p-2 text-sm" ref={ruleRef}>
            <Loader className="animate-spin" />
          </div>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  );
}

export default CombinedProxyRule;
