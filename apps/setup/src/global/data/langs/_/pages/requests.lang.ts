export type RequestsPageLang = {
  requestTabBar: {
    contextMenu: {
      closeAllPages: () => React.ReactNode;
      moveAllPagesToGroup: () => React.ReactNode;
    };
  };
  requestPages: {
    contextMenu: {
      pageNamePlaceholder: () => string;
      closeAll: () => React.ReactNode;
      closeAllOther: () => React.ReactNode;
      rename: () => React.ReactNode;
      duplicate: () => React.ReactNode;
      moveToGroup: () => React.ReactNode;
    };
  };
  requestMain: {
    urlPlaceholder: () => string;
    sendButton: () => React.ReactNode;
  };
  requestSettings: {
    tabs: {
      params: {
        name: () => React.ReactNode;
        queryParamsTitle: () => React.ReactNode;
        pathVariablesTitle: () => React.ReactNode;
      };
      headers: {
        name: () => React.ReactNode;
        headersTitle: () => React.ReactNode;
      };
    };
  };
};
