import { SectionCards } from '@/components/section-cards';
import { TrafficChart } from '@/components/traffic-chart';
import TrafficLog from '@/components/traffic-log';

export default function Dashboard() {
  return (
    <div className="flex flex-1 flex-col">
      <div className="flex flex-col gap-8">
        <SectionCards />
        <TrafficLog />
        <TrafficChart />
      </div>
    </div>
  );
}
