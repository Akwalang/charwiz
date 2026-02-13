import { isNaN } from '../is';

export type Comparator<T> = (a: T, b: T) => boolean;

export const isComparator = (worker: any): worker is Comparator<any> => {
  return typeof worker === 'function' && worker.length === 2;
};

export const DEFAULT_COMPARATOR: Comparator<any> = (a, b) => a === b || isNaN(a) && isNaN(b);
