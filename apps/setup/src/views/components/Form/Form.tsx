import { Input } from './components/Input/Input';
import { Select } from './components/Select/Select';
import { Button } from './components/Button/Button';
import { ContentEditable } from './components/ContentEditable/ContentEditable';
import { KeyValueEditor, KeyValueItem } from './components/KeyValueEditor/KeyValueEditor';

export { type KeyValueItem as KeyValueItem };

type Fields = {
  Input: typeof Input;
  Select: typeof Select;
  Button: typeof Button;
  ContentEditable: typeof ContentEditable;
  KeyValueEditor: typeof KeyValueEditor;
};

interface FormProps {}

export const Form: React.FC<FormProps> & Fields = (props) => {
  return <div>Form</div>;
};

Form.Input = Input;
Form.Select = Select;
Form.Button = Button;
Form.ContentEditable = ContentEditable;
Form.KeyValueEditor = KeyValueEditor;
