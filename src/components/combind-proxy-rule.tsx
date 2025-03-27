import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef } from 'react';
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

  useEffect(() => {
    loadProxyRules();
  }, []);

  return (
    <ScrollArea className="h-72 w-full rounded-md border">
      <div className="p-2 text-sm" ref={ruleRef}></div>
    </ScrollArea>
  );
}

export default CombinedProxyRule;
