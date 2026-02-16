import { Select } from "@/views/components";

import { useLang } from "@/global/hooks";

import { CommonSettings } from "@/global/stores/main-settings/types";
import { MainSettings } from "@/global/types/lang/settings/main";

import { getter } from "@/utils/object";
import { AllPaths, PathValue } from "@/utils/types";

type Selectable<T extends Record<any, any>> = {
  [K in keyof T as T[K] extends { values: Record<string, string> } ? K : never]: T[K];
};

interface SelectFieldProps<
  E extends CommonSettings,
  F extends keyof Selectable<MainSettings["fields"]>,
  P extends AllPaths<Exclude<E["editing"], null>>,
  V extends PathValue<E, P>,
> {
  entity: E,
  field: F,
  path: P,
  options: keyof Selectable<MainSettings["fields"]>[F]["values"],
  updateEntity: (id: string, path: P, value: V) => void,
}

export function SelectField<
  E extends CommonSettings,
  F extends keyof Selectable<MainSettings["fields"]>,
  P extends AllPaths<Exclude<E["editing"], null>>,
  V extends PathValue<E, P>,
>(props: SelectFieldProps<E, F, P, V>): React.ReactNode {
  const lang = useLang((state) => state.settings.main.fields);

  const items = Object.values(props.options).map((value: any) => {
    const name = (lang[props.field].values as any)[value];

    return { name, value };
  });

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        {lang[props.field].name}:
      </div>
      <div className="w-1/2">
        <Select
          items={items}
          value={getter(props.entity.editing!, props.path as any)! as string}
          onChange={(value) => props.updateEntity(props.entity.id, props.path, value as V)}
        />
      </div>
    </div>
  );
};
