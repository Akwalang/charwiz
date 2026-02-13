export const without = function without<
  T extends Record<any, any>,
  K extends keyof T,
>(target: T, key: K): Omit<T, K> {
  const result: T = { ...target };

  delete result[key];

  return result;
};
