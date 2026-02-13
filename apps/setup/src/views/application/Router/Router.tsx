import { MainLayout } from "@/views/layouts/MainLayout/MainLayout";
import { GeneralPage, HomePage, HotkeysPage } from '@/pages';

import { useRouterStore } from '@/global/stores/router/router.store';
import { Page } from '@/global/data/pages';

interface AppRouterProps {}

export const AppRouter: React.FC<AppRouterProps> = () => {
  const currentPage = useRouterStore((state) => state.page);

  const PageView = (() => {
    switch (currentPage) {
      case Page.Home: return HomePage;
      case Page.Hotkeys: return HotkeysPage;
      case Page.General: return GeneralPage;
      default: return HomePage;
    }
  })();

  return (
    <MainLayout>
      <PageView />
    </MainLayout>
  );
};
