const reg = /]\[|]\.|\.|]|\[/g;

enum Types {
  TYPE_OBJECT = 1,
  TYPE_ARRAY = 2,
}

type PathItem = { name: string, type: null | Types };

const parse = (path: string): PathItem[] => {
  const result: PathItem[] = [];

  let match: RegExpMatchArray | null = null;
  let name: string | null = null;
  let prev = 0;

  while ((match = reg.exec(path))) {
    const { 0: splitter, index } = match;

    name = path.slice(prev, index);

    if ((prev || name) && prev !== path.length) {
      const type = {
        '.': Types.TYPE_OBJECT,
        '[': Types.TYPE_ARRAY,
      }[splitter.slice(-1)] || null;

      result.push({ name, type });
    }

    prev = index! + splitter.length;
  }

  (name = path.slice(prev)) && result.push({ name, type: null });

  return result;
};

const createNode = (type: Types): Array<any> | Record<any, any> => ({
  [Types.TYPE_OBJECT]: {},
  [Types.TYPE_ARRAY]: [],
}[type]);

export const setter = function setter(target: Record<any, any> | Array<any> & Record<any, any>, path: string, value: any): void {
  let part: PathItem | undefined, context = target;

  const parts = parse(path);

  while ((part = parts.shift())) {
    const { name, type } = part;

    if (!name && Array.isArray(context)) {
      // push item in the end of array
      context = context[context.length] = parts.length && type ? createNode(type) : value;
    } else {
      context = context[name] = parts.length && type ? context[name] || createNode(type) : value;
    }
  }
};
