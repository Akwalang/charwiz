import { ThemeActions } from './slices/theme/actions';

export type Actions =
  | ThemeActions;

export const Actions = (set: any) => ({
  ...ThemeActions(set),
});
