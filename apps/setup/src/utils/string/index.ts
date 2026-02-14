export const snake2text = (str: string): string => {
  if (!str) return str;

  str = str.split('_').join(' ');

  return str[0].toUpperCase() + str.slice(1);
};
