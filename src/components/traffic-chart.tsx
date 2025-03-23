'use client';

import { Area, AreaChart, CartesianGrid, XAxis } from 'recharts';

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from '@/components/ui/chart';
const chartData = [
  { month: 'January', downlink: 186, uplink: 80 },
  { month: 'February', downlink: 305, uplink: 200 },
  { month: 'March', downlink: 237, uplink: 120 },
  { month: 'April', downlink: 73, uplink: 190 },
  { month: 'May', downlink: 209, uplink: 130 },
  { month: 'June', downlink: 214, uplink: 140 },
];

const chartConfig = {
  downlink: {
    label: 'downlink',
    color: 'var(--chart-1)',
  },
  uplink: {
    label: 'uplink',
    color: 'var(--chart-2)',
  },
} satisfies ChartConfig;

export function TrafficChart() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Area Chart - Stacked</CardTitle>
        <CardDescription>
          Showing total visitors for the last 6 months
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig}>
          <AreaChart
            accessibilityLayer
            data={chartData}
            margin={{
              left: 8,
              right: 8,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="month"
              tickMargin={8}
              tickFormatter={(value) => value.slice(0, 3)}
            />
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent indicator="dot" />}
            />
            <Area
              dataKey="uplink"
              type="natural"
              fill="var(--color-uplink)"
              fillOpacity={0.4}
              stroke="var(--color-uplink)"
              stackId="a"
            />
            <Area
              dataKey="downlink"
              type="natural"
              fill="var(--color-downlink)"
              fillOpacity={0.4}
              stroke="var(--color-downlink)"
              stackId="a"
            />
          </AreaChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
