import { Page } from '@/global/data/pages';

export type State = {
  page: Page;
};

export const State = (): State => ({
  page: Page.Home,
}) as const;
