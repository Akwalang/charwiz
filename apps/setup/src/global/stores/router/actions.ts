import { navigate } from './actions/navigate';

export const Actions = (set: any) => ({
  navigate: navigate(set),
}) as const;

export type Actions = ReturnType<typeof Actions>;
