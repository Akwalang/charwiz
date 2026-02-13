import { useEffect } from 'react';

import { AppRouter } from '@/views/application/Router/Router';
import { Theme } from '@/views/application/Theme/Theme';

import './App.css';

interface AppProps {}

const preventContextMenu = (event: MouseEvent) => {
  // event.preventDefault();
};

export const App: React.FC<AppProps> = () => {
  useEffect(() => {
    window.addEventListener('contextmenu', preventContextMenu);

    return () => {
      window.removeEventListener('contextmenu', preventContextMenu);
    };
  }, []);

  return (
    <>
      <Theme />
      <AppRouter />
    </>
  );
};
