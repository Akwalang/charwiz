const getType = (val: unknown): string => Object.prototype.toString.call(val).slice(8, -1);

export const isSuperSet = (sup: any, sub: any): boolean => {
  if (getType(sup) !== getType(sub)) return false;
  
  if (typeof sup !== 'object' || sup === null) {
    return sup === sub;
  }

  for (const field in sub) {
    if (!sub.hasOwnProperty(field)) continue;

    if (!(field in sup) || !isSuperSet(sup[field], sub[field])) {
      return false;
    }
  }

  return true;
};
