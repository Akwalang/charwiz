export const idxGuard = <
  Id extends string | number,
  Tp extends { id: Id },
  >(name: string, list: Tp[], id: Id): number => {
  const idx = list.findIndex((item) => item.id === id);

  if (idx === -1) {
    throw new Error(`${name} does not exist: id=${id}`);
  }

  return idx;
};
