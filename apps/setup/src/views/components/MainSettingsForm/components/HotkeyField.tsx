import { ContentEditable } from "@/views/components";

import { useLang } from "@/global/hooks";
import { useHotkeyCapture } from "@/global/hooks/useHotkeyCapture";

import { KeyboardSnapshot } from "@/global/types/settings/types";

import { CommonSettings } from "@/global/stores/main-settings/types";
import { MainSettings } from "@/global/types/lang/settings/main";

import { getter } from "@/utils/object";
import { AllPaths, PathValue } from "@/utils/types";

type WithAction<T extends Record<any, any>> = {
  [K in keyof T as T[K] extends { action: string } ? never : K]: T[K];
};

interface InputFieldProps<
  E extends CommonSettings,
  F extends keyof WithAction<MainSettings["fields"]>,
  P extends AllPaths<Exclude<E["editing"], null>>,
  V extends KeyboardSnapshot,
> {
  entity: E,
  field: F,
  path: P,
  updateEntity: (id: string, path: P, value: V) => void,
}

export function HotkeyField<
  E extends CommonSettings,
  F extends keyof WithAction<MainSettings["fields"]>,
  P extends AllPaths<Exclude<E["editing"], null>>,
  V extends KeyboardSnapshot,
>(props: InputFieldProps<E, F, P, V>): React.ReactNode {
  const lang = useLang((state) => state.settings.main.fields);

  const { isRecording, startListen, stopListen } = useHotkeyCapture(props.entity.id, props.path, props.updateEntity as any);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        {lang[props.field].name}:
      </div>
      <div className="w-1/2">
        <div className="h-[32px] bg-amber-400" onClick={() => isRecording ? stopListen() : startListen()}>{isRecording ? "Recording" : "Sleeping"}</div>
      </div>
    </div>
  );
};
