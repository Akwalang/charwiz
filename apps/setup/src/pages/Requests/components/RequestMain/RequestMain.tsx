import { memo } from 'react';
import { ChevronDown } from 'lucide-react';

import { PageRequest } from '@/global/stores/workspace/slices/page-request/types';
import { useWorkspaceStore } from '@/global/stores/workspace';

import { Breadcrumb, Form } from '@/views/components';

import { useLang, useInput } from '@/global/hooks';
import { HttpMethod } from '@/global/data/http';

import { cn } from '@/utils/react';

interface RequestMainProps extends PageRequest {}

const methods = Object.values(HttpMethod).map((value) => (
  { name: value.toUpperCase(), value }
));

export const RequestMain: React.FC<RequestMainProps> = memo((props) => {
  const { request } = props;

  const lang = useLang((store) => store.pages.requests.requestMain);

  const changeRequestMethod = useWorkspaceStore((state) => state.changeRequestMethod);
  const changeRequestUrl = useWorkspaceStore((state) => state.changeRequestUrl);

  const onInputChange = useInput(changeRequestUrl);

  const breadcrumb = [
    { name: 'Item 1' },
    { name: 'Item 2' },
    { name: 'Item 3' },
    { name: 'Item 4' },
    { name: 'Item 5' },
    { name: 'Item 6' },
  ];

  return (
    <div className="px-[15px]">
      <div className="flex flex-col py-[10px]">
        <Breadcrumb items={breadcrumb} />
      </div>

      <div className="flex gap-2">
        <Form.Select className="min-w-[110px] max-w-[110px] text-center select-none" value={request.method} items={methods} onChange={(method) => changeRequestMethod(method as HttpMethod)}  />
        <Form.Input className="grow" value={request.url} onChange={onInputChange} placeholder={lang.urlPlaceholder()} />
        <div className="relative flex select-none">
          <Form.Button type="submit">{lang.sendButton()}</Form.Button>
          <Form.Button size="sm" variant="secondary" className={cn(
            "absolute top-[-10px] right-[-10px]",
            "w-[20px] h-[20px]",
          )}>
            <ChevronDown size={20} />
          </Form.Button>
        </div>
      </div>
    </div>
  );
});
