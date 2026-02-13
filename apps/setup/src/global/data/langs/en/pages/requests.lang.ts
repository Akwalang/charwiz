import { RequestsPageLang } from "@/global/data/langs/_/pages/requests.lang";

export const requests: RequestsPageLang = {
  requestTabBar: {
    contextMenu: {
      closeAllPages: () => 'Close all pages',
      moveAllPagesToGroup: () => 'Move all pages to group',
    },
  },
  requestPages: {
    contextMenu: {
      pageNamePlaceholder: () => 'Set page name',
      closeAll: () => 'Close all pages',
      closeAllOther: () => 'Close all other pages',
      rename: () => 'Rename page',
      duplicate: () => 'Duplicate page',
      moveToGroup: () => 'Move page to group',
    },
  },
  requestMain: {
    urlPlaceholder: () => 'Set URL address',
    sendButton: () => 'Send',
  },
  requestSettings: {
    tabs: {
      params: {
        name: () => 'Params',
        queryParamsTitle: () => 'Query params',
        pathVariablesTitle: () => 'Path variables',
      },
      headers: {
        name: () => 'Headers',
        headersTitle: () => 'Headers',
      },
    },
  },
};
