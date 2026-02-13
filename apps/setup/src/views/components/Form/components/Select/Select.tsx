import * as Base from "@/views/ui/select";

import { cn } from '@/utils/react';

type Item = {
  name: string;
  value: string;
};

type Group = {
  name: string;
  items: Item[];
};

interface SelectProps extends React.RefAttributes<HTMLButtonElement> {
  onChange?: (value: string) => void;
  className?: string;
  value?: string;
  placeholder?: string;
  items?: Item[];
  groups?: Group[];
}

const Items: React.FC<{ items: Item[] }> = (props) => {
  const result = [];

  for (const [i, item] of props.items.entries()) {
    result.push(<Base.SelectItem key={'i' + i} value={item.value}>{item.name}</Base.SelectItem>);
  }

  return result;
};

const Groups: React.FC<{ groups: Group[] }> = (props) => {
  const result = [];

  for (const [i, group] of props.groups.entries()) {
    result.push(
      <Base.SelectGroup key={'g' + i}>
        <Base.SelectLabel>{group.name}</Base.SelectLabel>
        <Items items={group.items}></Items>
      </Base.SelectGroup>
    );
  }

  return result;
};

export const Select: React.FC<SelectProps> = (props) => {
  const { value, placeholder, items, groups, className, onChange, ...other } = props;

  const children = [];

  items?.length && children.push(<Items key="i" items={items} />);
  groups?.length && children.push(<Groups key="g" groups={groups} />);

  return (
    <Base.Select value={props.value} onValueChange={onChange}>
      <Base.SelectTrigger className={cn('text-left [&>span]:grow', className)} {...other}>
        <Base.SelectValue placeholder={placeholder} />
      </Base.SelectTrigger>
      <Base.SelectContent>
        {children}
      </Base.SelectContent>
    </Base.Select>
  )
}
