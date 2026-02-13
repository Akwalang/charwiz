import { memo, forwardRef, useRef, useEffect, useState } from "react";

import * as Base from '@/views/ui/dropdown-menu';

import { isEqual } from "@/utils/is";
import { cn } from "@/utils/react";

import { empower } from "./utils/empower.utils";
import { filterSuggestions } from "./utils/filter-suggestions.utils";

interface ContentEditableProps {
  className?: string;
  fieldClassName?: string;
  value: string;
  multiline?: boolean;
  placeholder?: string;
  options?: string[];
  disabled?: boolean;
  stopPropagation?: boolean;
  onChange: (value: string) => void;
  onFocus?: () => void;
  onBlur?: () => void;
}

export const ContentEditable: React.FC<ContentEditableProps> = memo((props) => {
  const [value, setValue] = useState(props.value);
  const [isOpen, setIsOpen] = useState(false);
  const [suggestions, setSuggestions] = useState<string[]>(filterSuggestions(value, props.options));

  const field = useRef<HTMLDivElement>(null);

  const stopPropagation = (event: React.MouseEvent<HTMLDivElement>) => {
    props.stopPropagation && event.stopPropagation();
  };

  const onChange = (value: string) => {
    props.onChange?.(value.trim());

    const list = filterSuggestions(value, props.options);

    setValue(value);
    setSuggestions(list);
    setIsOpen(!!list.length);
  };

  const onFocus = () => {
    setIsOpen(!!suggestions.length);
  };

  const onSelect = (value: string) => {
    field.current!.textContent = value;

    props.onChange?.(value.trim());

    setValue(value);
    setSuggestions(filterSuggestions(value, props.options));
    setIsOpen(false);
  };

  useEffect(() => {
    const { multiline } = props;

    empower(field.current!, { multiline });
  }, [props.multiline]);

  return (
    <div className={cn("relative", props.className)}>
      <Base.DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
        <Base.DropdownMenuTrigger asChild>
          <div className={cn(
            props.fieldClassName,
            "absolute top-0 left-0 w-full h-full pointer-events-none select-none",
            value === '' || value === '<br>' ? "text-foreground/40" : "text-transparent",
          )}>
            {props.placeholder}
          </div>
        </Base.DropdownMenuTrigger>
        {!!suggestions.length && <Options value={value} options={suggestions} onSelect={onSelect} />}
      </Base.DropdownMenu>
        <Field
          ref={field}
          value={props.value}
          key="contenteditable"
          className={cn(
            props.fieldClassName,
            "w-full h-full cursor-text",
            "focus:outline-0 focus:relative focus:border-primary focus:z-1",
          )}
          disabled={props.disabled || false}
          onClick={stopPropagation}
          onMouseDown={stopPropagation}
          onInput={() => onChange(field.current!.textContent ?? "")}
          onFocus={() => onFocus()}
        />
    </div>
  );
}, isEqual);

interface FieldProps {
  ref: React.Ref<HTMLDivElement>;
  value: string;
  disabled: boolean;
  className: string;
  onClick: (event: React.MouseEvent<HTMLDivElement>) => void;
  onMouseDown: (event: React.MouseEvent<HTMLDivElement>) => void;
  onInput: () => void;
  onFocus: () => void;
}

const Field = memo(forwardRef<HTMLDivElement, FieldProps>((props, ref) => {
  const [inValue, setInValue] = useState(props.value);

  const { value, disabled, ...rest } = props;

  useEffect(() => {
    const propValue = (props.value || '').trim();
    const refValue = ((ref as any).current.textContent || '').trim();

    propValue !== refValue && setInValue(props.value);
  }, [props.value]);

  return (
    <div
      ref={ref}
      key="contenteditable"
      contentEditable={!props.disabled}
      suppressContentEditableWarning={true}
      children={inValue}
      {...rest}
    />
  );
}), isEqual);

interface OptionsProps {
  value: string;
  options: string[];
  onSelect: (option: string) => void;
}

const Options: React.FC<OptionsProps> = (props) => {
  const preventAutoFocus = {
    onCloseAutoFocus: (e: Event) => e.preventDefault(),
    onOpenAutoFocus: (e: Event) => e.preventDefault(),
  };

  return (
    <Base.DropdownMenuPortal>
      <Base.DropdownMenuContent
        className="w-[var(--radix-dropdown-menu-trigger-width)]"
        area-hidden="false"
        {...preventAutoFocus}
      >
        {props.options.map((option) => (
          <Base.DropdownMenuItem
            key={option}
            className="cursor-pointer"
            onClick={() => props.onSelect(option)}
          >
            {option}
          </Base.DropdownMenuItem>
        ))}
      </Base.DropdownMenuContent>
    </Base.DropdownMenuPortal>
  );
};
