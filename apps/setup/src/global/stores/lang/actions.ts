import { setLang } from './actions/set-lang';

export const Actions = (set: any) => ({
  setLang: setLang(set),
}) as const;

export type Actions = ReturnType<typeof Actions>;
