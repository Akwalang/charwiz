export type ArrayToTuple<T extends any[]> = {
  [K in keyof T]: number | null;
};
