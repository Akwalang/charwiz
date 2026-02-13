import { type Comparator, isComparator, DEFAULT_COMPARATOR } from './common';

type Getter<T> = (item: T) => any;

const isGetter = (worker: Function): worker is Getter<any> => worker.length === 1;

function difference<T>(a: T[], b: T[], worker?: Getter<T>): T[];
function difference<T>(a: T[], b: T[], worker?: Comparator<T>): T[];
function difference<T>(a: T[], b: T[], worker: Getter<T> | Comparator<T> = DEFAULT_COMPARATOR): T[] {
  let comparator: Comparator<T>;

  if (isComparator(worker)) {
    comparator = worker;
  }

  if (isGetter(worker)) {
    comparator = (a, b) => DEFAULT_COMPARATOR(worker(a), worker(b));
  }

  return a.filter(av => !b.find(bv => comparator(av, bv)));
};

export { difference };
