export type ButtonsSettings = {
  edit: string,
  save: string,
  cancel: string,
  delete: string,
  waiting: (remain: number) => React.ReactNode,
};
