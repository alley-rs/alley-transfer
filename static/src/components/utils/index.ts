export const addClassNames = (
  base: string,
  ...others: (string | undefined | false)[]
): string => {
  const names = Array.from(new Set(others.filter((s) => s && s !== "")));
  return [base, ...names].join(" ");
};
