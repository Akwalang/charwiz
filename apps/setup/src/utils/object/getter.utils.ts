import { AllPaths, PathValue } from "../types/paths.type";

export const getter = function getter<
  T extends object,
  P extends AllPaths<T>,
>(data: T, path: P): PathValue<T, P> | undefined {
  let name, context: any = data;

  const paths = path.split(/]\[|]\.|\.|]|\[/).filter(v => v);

  while ((name = paths.shift())) {
    if (context == null) break;

    context = context[name];
  }

  return context;
};
