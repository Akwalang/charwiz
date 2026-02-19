import { Logo, LangSwitcher, ThemeSwitcher } from '@/views/components';

import { Navigation } from './components/Navigation/Navigation';

import { cn } from '@/utils/react';

interface MainLayoutProps extends React.ComponentProps<'div'> {}

export const MainLayout: React.FC<MainLayoutProps> = ({ children, className, ...other }) => {
  return (
    <div
      className={cn(
        "w-full flex flex-col items-center bg-background/50",
        "bg-linear-to-r from-sidebar from-49% to-background to-51%",
        className,
      )}
      {...other}
    >
      <div className="w-full min-h-svh max-w-6xl flex bg-background shadow-xl">
        <div className="w-full flex">
          <aside className="w-64 bg-sidebar border-r relative select-none">
            <div className="w-full p-6 sticky top-0">
              <Logo />
              <div className="mt-4">
                <ThemeSwitcher />
              </div>
              <div className="mt-4">
                <LangSwitcher />
              </div>
              <hr className="my-6" />
              <Navigation />
            </div>
          </aside>

          <main className="relative flex-1 p-8">
            {children}
          </main>
        </div>
      </div>
    </div>
  );
};
