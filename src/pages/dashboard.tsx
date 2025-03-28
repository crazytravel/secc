import { SectionCards } from '@/components/section-cards';
import { TrafficChart } from '@/components/traffic-chart';

export default function Dashboard() {
  return (
    <div className="flex flex-1 flex-col">
      <div className="flex flex-col gap-8">
        <SectionCards />
        <TrafficChart />
      </div>
    </div>
  );
}
