export type FormComponentLang = {
  keyValueEditor: {
    columns: {
      key: () => React.ReactNode;
      value: () => React.ReactNode;
      description: () => React.ReactNode;
    };
  };
};
