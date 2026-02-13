import { Separator as BaseSeparator } from "@/views/ui/separator";

interface SeparatorProps {
  full?: boolean;
}

export const Separator: React.FC<SeparatorProps> = (props) => {
  return <BaseSeparator className={props.full ? "h-full" : "h-[45%]"} orientation="vertical" />
};
