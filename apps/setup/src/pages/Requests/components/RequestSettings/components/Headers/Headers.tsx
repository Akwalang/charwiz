import { Form, KeyValueItem } from '@/views/components';

import { HttpHeadersAutocomplete } from '@/global/data/http';
import { useLang } from '@/global/hooks/useLang';
import { useWorkspaceStore } from '@/global/stores/workspace';

interface HeadersProp {
  headers: KeyValueItem[];
  fieldClassName?: string;
}

export const Headers: React.FC<HeadersProp> = (props) => {
  const lang = useLang((store) => store.pages.requests.requestSettings.tabs.headers);
  const changeRequestHeaders = useWorkspaceStore((state) => state.changeRequestHeaders);

  const headers = props.headers.map((item) => {
    return {
      ...item,
      keyOptions: HttpHeadersAutocomplete.names || [],
      valueOptions: HttpHeadersAutocomplete.values[item.key[0].toLowerCase()] || [],
    };
  });

  return (
    <div className="flex flex-col px-[15px] pb-[15px] gap-[8px]">
      <div>
        <h4 className="mb-2 text-sm select-none">{lang.headersTitle()}</h4>
        <Form.KeyValueEditor
          extendable={true}
          items={headers}
          fieldClassName={props.fieldClassName}
          onChange={changeRequestHeaders}
        />
      </div>
    </div>
  );
};
