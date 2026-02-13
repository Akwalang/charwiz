import { langs } from '@/global/data/langs';
import { LangCode } from '@/global/data/langs/enums';

export type State = {
  lang: keyof typeof langs;
};

export const State = (): State => ({
  lang: LangCode.By,
}) as const;
