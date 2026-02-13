import { memo } from 'react';

import { PageRequest } from '@/global/stores/workspace/slices/page-request/types';

import { useLang } from '@/global/hooks';

import { Tabs } from '@/views/components';

import { Params } from './components/Params/Params';
import { Headers } from './components/Headers/Headers';

interface RequestSettingsProps extends PageRequest {}

export const RequestSettings: React.FC<RequestSettingsProps> = memo((props) => {
  const lang = useLang((store) => store.pages.requests.requestSettings.tabs);

  const fieldClassName = 'hover:bg-primary/10 focus:bg-primary/10';

  const tabs = [
    {
      name: lang.params.name(),
      value: 'params',
      content: () => <Params {...props.request.params} fieldClassName={fieldClassName} />,
    },
    {
      name: lang.headers.name(),
      value: 'headers',
      content: () => <Headers headers={props.request.headers} fieldClassName={fieldClassName} />,
    },
  ];

  return (
    <div className="h-full">
      <Tabs className="px-[15px]" tabs={tabs} defaultValue={tabs[0].value} scrollable={true} />
    </div>
  );
});
