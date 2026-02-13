import { Form, KeyValueItem } from '@/views/components';

import { useWorkspaceStore } from '@/global/stores/workspace';
import { useLang } from '@/global/hooks/useLang';

interface ParamsProp {
  path: KeyValueItem[];
  query: KeyValueItem[];
  fieldClassName?: string;
}

export const Params: React.FC<ParamsProp> = (props) => {
  const lang = useLang((store) => store.pages.requests.requestSettings.tabs.params);

  const changeRequestPathParams = useWorkspaceStore((state) => state.changeRequestPathParams);
  const changeRequestQueryParams = useWorkspaceStore((state) => state.changeRequestQueryParams);

  return (
    <div className="flex flex-col px-[15px] pb-[15px] gap-[8px]">
      { !!props.path.length &&
        <div>
          <h4 className="mb-2 text-sm select-none">{lang.pathVariablesTitle()}</h4>
          <Form.KeyValueEditor
            items={props.path}
            fieldClassName={props.fieldClassName}
            onChange={changeRequestPathParams}
          />
        </div>
      }
      <div>
        <h4 className="mb-2 text-sm select-none">{lang.queryParamsTitle()}</h4>
        <Form.KeyValueEditor
          extendable={true}
          items={props.query}
          fieldClassName="hover:bg-primary/10 focus:bg-primary/10"
          onChange={changeRequestQueryParams}
        />
      </div>
    </div>
  );
};
