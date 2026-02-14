export const deleteById = <
  Id extends string | number,
  Tp extends { id: Id },
>(list: Tp[], id: Id): Tp[] => {
  return list.filter((item) => item.id !== id);
};
