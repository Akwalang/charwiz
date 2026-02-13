import { DeepPartial } from '../types';

import { merge } from './merge.utils';

export const replace = <T extends Record<any, any>>(obj: T, update: DeepPartial<T>): T => {
  return merge({ ...obj }, update);
};
