import { Separator } from '@/views/ui/separator';

import { RequestLayout } from '../RequestLayout/RequestLayout';

import { Tabs } from '../../components/Tabs/Tabs';

export const TabsLayout: React.FC = () => {
  return (
    <div className="flex flex-col items-stretch h-full">
      <Tabs />
      <Separator className="my-0" />
      <RequestLayout />
    </div>
  );
};
