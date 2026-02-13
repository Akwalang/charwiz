import { LangCode } from './enums';
import { type LangType } from './_';

import { en } from './en';
import { by } from './by';

export { LangCode } from './enums';

export const langs: Record<LangCode, LangType> = { en, by };
