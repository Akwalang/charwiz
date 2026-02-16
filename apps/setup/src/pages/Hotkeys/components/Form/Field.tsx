import { Select } from "@/views/components";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";
import { HotkeyItem } from "@/global/stores/main-settings/state";

import { MainSettings } from "@/global/types/lang/settings/main";

import { getter } from "@/utils/object";
import { FullPaths } from "@/utils/types";

type Selectable<T extends Record<any, any>> = {
  [K in keyof T as T[K] extends { values: Record<string, string> } ? K : never]: T[K];
};

interface FieldProps<
  F extends keyof Selectable<MainSettings["fields"]>
> {
  id: string;
  field: F;
  path: FullPaths<Exclude<HotkeyItem["editing"], null>>;
  options: keyof Selectable<MainSettings["fields"]>[F]["values"];
}

export function Field<
  F extends keyof Selectable<MainSettings["fields"]>
>(props: FieldProps<F>): React.ReactNode {
  const lang = useLang((state) => state.settings.main.fields);
  const hotkey = useMainSettingsStore(getHotkey(props.id));

  const updateHotkey = useMainSettingsStore((state) => state.updateHotkey);

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
          value={getter(hotkey!.editing!, props.path)! as string}
          onChange={(value) => updateHotkey(props.id, props.path, value)}
        />
      </div>
    </div>
  );
};

// interface FieldProps {
//   id: string,
//   field: keyof Selectable<MainSettings["fields"]>,
//   path: FullPaths<Exclude<HotkeyItem["editing"], null>>,
//   options: Record<string, string>,
// };

// export const Field: React.FC<FieldProps> = (props) => {
//   const lang = useLang((state) => state.settings.main.fields);
//   const hotkey = useMainSettingsStore(getHotkey(props.id));

//   const updateHotkey = useMainSettingsStore((state) => state.updateHotkey);

//   const items = Object.values(props.options).map((value: any) => {
//     const name = lang[props.field].values[value as any];

//     return { name, value };
//   });

//   return (
//     <div className="flex items-center py-3">
//       <div className="w-1/2">
//         {lang[props.field].name}:
//       </div>
//       <div className="w-1/2">
//         <Select
//           items={items}
//           value={getter(hotkey!.editing!, props.path)!}
//           onChange={(value) => updateHotkey(props.id, props.path, value)}
//         />
//       </div>
//     </div>
//   );
// };

// const UserInputCleanup: React.FC<OptionProps> = (props) => {
//   const items = enum2items(UserInputCleanupEnum);

//   return (
//     <div className="flex items-center py-3">
//       <div className="w-1/2">
//         User input cleanup:
//       </div>
//       <div className="w-1/2">
//         <Form.Select
//           items={items}
//           value={props.values.settings.injector.userInputCleanup}
//           onChange={(value) => props.onUpdate(props.id, 'settings.injector.userInputCleanup', value)}
//         />
//       </div>
//     </div>
//   );
// };
