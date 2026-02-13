import { Sun, Moon, Paintbrush, Check, ArrowRightFromLine } from 'lucide-react';

import { themes, ThemeName } from '@/global/data/themes';

import { useThemeStore } from '@/global/stores/theme';

import * as Base from '@/views/ui/dropdown-menu';
import { Button } from '@/views/ui/button';

interface ThemeSwitcherProps {}

const Options: React.FC<{ cur: string }> = (props) => {
  const setTheme = useThemeStore((state) => state.setTheme);

  return Object.values(ThemeName).map((name) => {
    const theme = themes[name];

    return (
      <Base.DropdownMenuItem key={name} onClick={() => setTheme(name)}>
        <div className="w-full flex items-center justify-between">
          <span>{theme.label}</span>
          <span>{props.cur === name && <Check size="16" />}</span>
        </div>
      </Base.DropdownMenuItem>
    );
  });
};

export const ThemeSwitcher: React.FC<ThemeSwitcherProps> = () => {
  const { mode, theme, toggleMode, nextTheme } = useThemeStore((state) => state);

  const selected = themes[theme] || { label: 'Unknown' };

  const ModeIcon = mode === 'dark' ? Moon : Sun;

  const iconClassName = "h-full flex p-0.5 shadow-none bg-transparent focus:outline-none hover:no-underline";

  return (
    <div className="h-full items-center flex select-none">
      <Button variant="link" size="sm" className={iconClassName} onClick={toggleMode}>
        <ModeIcon className="text-foreground" size={14} />
      </Button>
      <Button variant="link" size="sm" className={iconClassName} onClick={nextTheme}>
        <ArrowRightFromLine className="text-foreground" size={14} />
      </Button>
      <Base.DropdownMenu>
        <Base.DropdownMenuTrigger asChild>
          <Button variant="ghost" size="sm" className={iconClassName}>
            <Paintbrush className="text-foreground" size={14} />
            <div className="text-xs text-foreground ml-[-3px]">{selected.label}</div>
          </Button>
        </Base.DropdownMenuTrigger>
        <Base.DropdownMenuContent align="center" className="w-[160px]">
          <Options cur={theme} />
        </Base.DropdownMenuContent>
      </Base.DropdownMenu>
    </div>
  );
};
