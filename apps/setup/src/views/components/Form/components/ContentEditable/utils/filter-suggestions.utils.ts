export const filterSuggestions = (value: string, options: string[] | undefined | null): string[] => {
  const val = value.toLowerCase();

  return (options || []).filter((option) => {
    option = option.toLowerCase();

    return option.includes(val) && option !== val;
  });
};
