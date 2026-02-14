import { Form } from "@/views/components/Form/Form";

import { CommonValues } from "@/global/types/settings/settings/main-settings";

import {
  UserInputCleanupEnum,
  KeyboardStateCleanupEnum,
  KeyboardLayoutEnum,
  TransformTargetEnum,
  InjectMethodEnum,
} from "@/global/types/settings/enums";

import { snake2text } from "@/utils/string";

interface OptionsProps {
  id: string,
  values: {
    title: string,
    description: string,
    settings: CommonValues,
  },
  onUpdate: (id: string) => void,
}

export const Options: React.FC<OptionsProps> = (props) => {
  return (
    <Form>
      <hr className="opacity-60 border-dashed" />
      <UserInputCleanup {...props} />
      <hr className="opacity-60 border-dashed" />
      <KeyboardStateCleanup {...props} />
      <hr className="opacity-60 border-dashed" />
      <LayoutBefore {...props} />
      <hr className="opacity-60 border-dashed" />
      <LayoutAfter {...props} />
      <hr className="opacity-60 border-dashed" />
      <TransformTarget {...props} />
      <hr className="opacity-60 border-dashed" />
      <InjectMethod {...props} />
    </Form>
  );
};

const enum2items = (enums: Record<string, string>): { name: string, value: string }[] => {
  return Object.values(enums).map((item) => ({
    name: snake2text(item),
    value: item,
  }));
};

interface OptionProps {
  id: string,
  values: {
    title: string,
    description: string,
    settings: CommonValues,
  },
  onUpdate: (id: string, path: string, value: any) => void,
}

const UserInputCleanup: React.FC<OptionProps> = (props) => {
  const items = enum2items(UserInputCleanupEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        User input cleanup:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.userInputCleanup}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.userInputCleanup', value)}
        />
      </div>
    </div>
  );
};

const KeyboardStateCleanup: React.FC<OptionProps> = (props) => {
  const items = enum2items(KeyboardStateCleanupEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        Keyboard state cleanup:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.keyboardStateCleanup}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.keyboardStateCleanup', value)}
        />
      </div>
    </div>
  );
};

const LayoutBefore: React.FC<OptionProps> = (props) => {
  const items = enum2items(KeyboardLayoutEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        Layout before:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.layoutBefore}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.layoutBefore', value)}
        />
      </div>
    </div>
  );
};

const LayoutAfter: React.FC<OptionProps> = (props) => {
  const items = enum2items(KeyboardLayoutEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        Layout after:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.layoutAfter}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.layoutAfter', value)}
        />
      </div>
    </div>
  );
};

const TransformTarget: React.FC<OptionProps> = (props) => {
  const items = enum2items(TransformTargetEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        Transform target:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.target}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.target', value)}
        />
      </div>
    </div>
  );
};

const InjectMethod: React.FC<OptionProps> = (props) => {
  const items = enum2items(InjectMethodEnum);

  return (
    <div className="flex items-center py-3">
      <div className="w-1/2">
        Inject method:
      </div>
      <div className="w-1/2">
        <Form.Select
          items={items}
          value={props.values.settings.injector.method}
          onChange={(value) => props.onUpdate(props.id, 'settings.injector.method', value)}
        />
      </div>
    </div>
  );
};
