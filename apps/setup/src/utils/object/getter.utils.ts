export const getter = function getter<T = any>(data: object, path: string): T | undefined {
  let name, context: any = data;

  const paths = path.split(/]\[|]\.|\.|]|\[/).filter(v => v);

  while ((name = paths.shift())) {
    if (context == null) break;

    context = context[name];
  }

  return context as T;
};
