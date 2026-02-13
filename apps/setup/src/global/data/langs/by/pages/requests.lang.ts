import { RequestsPageLang } from "../../_/pages/requests.lang";

export const requests: RequestsPageLang = {
  requestTabBar: {
    contextMenu: {
      closeAllPages: () => 'Закрыць усе старонкі',
      moveAllPagesToGroup: () => 'Перамясціць усе старонкі ў групу',
    },
  },
  requestPages: {
    contextMenu: {
      pageNamePlaceholder: () => 'Увядзіце назву старонкі',
      closeAll: () => 'Закрыць усе старонкі',
      closeAllOther: () => 'Закрыць усе іншыя старонкі',
      rename: () => 'Перайменаваць старонку',
      duplicate: () => 'Дубляваць старонку',
      moveToGroup: () => 'Перамясціць старонку ў групу',
    },
  },
  requestMain: {
    urlPlaceholder: () => 'Увядзіце URL',
    sendButton: () => 'Адправіць',
  },
  requestSettings: {
    tabs: {
      params: {
        name: () => 'Параметры',
        queryParamsTitle: () => 'Параметры запыту',
        pathVariablesTitle: () => 'Пераменныя шляха',
      },
      headers: {
        name: () => 'Загалоўкі',
        headersTitle: () => 'Загалоўкі',
      },
    },
  },
};
