import { ContentEditable } from "@/views/components";

import { useLang } from "@/global/hooks";

import { CommonSettings } from "@/global/stores/main-settings/types";
import { MainSettings } from "@/global/types/lang/settings/main";

import { getter } from "@/utils/object";
import { FullPaths, PathValue } from "@/utils/types";

type NotSelectable<T extends Record<any, any>> = {
  [K in keyof T as T[K] extends { values: Record<string, string> } ? never : K]: T[K];
};

interface InputFieldProps<
  E extends CommonSettings,
  F extends keyof NotSelectable<MainSettings["fields"]>,
  P extends FullPaths<Exclude<E["editing"], null>>,
  V extends PathValue<E, P>,
> {
  entity: E,
  field: F,
  path: P,
  updateEntity: (id: string, path: P, value: V) => void,
}

export function InputField<
  E extends CommonSettings,
  F extends keyof NotSelectable<MainSettings["fields"]>,
  P extends FullPaths<Exclude<E["editing"], null>>,
  V extends PathValue<E, P>,
>(props: InputFieldProps<E, F, P, V>): React.ReactNode {
  const lang = useLang((state) => state.settings.main.fields);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        {lang[props.field].name}:
      </div>
      <div className="w-1/2">
        <ContentEditable
          value={getter(props.entity.editing!, props.path as any)! as string}
          onChange={(value) => props.updateEntity(props.entity.id, props.path, value as V)}
        />
      </div>
    </div>
  );
};
