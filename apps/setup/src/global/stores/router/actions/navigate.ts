import { Page } from '@/global/data/pages';

export const navigate = (set: any) => (page: Page) => {
  set(() => ({ page }));
};
