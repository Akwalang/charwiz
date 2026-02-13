import { useWorkspaceStore } from '@/global/stores/workspace';
import { getActiveGroup } from '@/global/stores/workspace/selectors';

import { MainLayout } from './layouts/MainLayout/MainLayout';
import { TabsLayout } from './layouts/TabsLayout/TabsLayout';

import { Groups } from './components/Groups/Groups';
import { Collection } from './components/Collection/Collection';
import { EmptyGroup } from './components/EmptyGroup/EmptyGroup';

interface RequestsPageProps {}

export const RequestsPage: React.FC<RequestsPageProps> = () => {
  const group = useWorkspaceStore(getActiveGroup);

  const groups = <Groups />;
  const collection = <Collection />;

  const workingArea = group.pageIds.length ? <TabsLayout /> : <EmptyGroup />;

  return <MainLayout groups={groups} collection={collection} workingArea={workingArea} />;
};
