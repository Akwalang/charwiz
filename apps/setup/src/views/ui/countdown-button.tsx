import { useRef, useState } from "react";

import { Button, ButtonProps } from "./button";

interface CountdownButtonProps extends ButtonProps {
  countdown?: number,
  tickSize?: number,
  onTimeUp: () => void,
  waitingText: (v: number) => React.ReactNode,
}

const DEFAULT_COUNTDOWN = 3;
const DEFAULT_TICK_SIZE = 1000;

export const CountdownButton: React.FC<CountdownButtonProps> = ({
  countdown = DEFAULT_COUNTDOWN,
  tickSize = DEFAULT_TICK_SIZE,
  onTimeUp,
  waitingText,
  children,
  ...other
}) => {
  const { isActive, remain, startTimer } = useCountdown(countdown, tickSize, onTimeUp);

  const onMouseDown: React.ReactEventHandler = (event) => {
    if (countdown <= 0) return;

    event.preventDefault();

    startTimer('mouseup');
  };

  const onKeyDown: React.KeyboardEventHandler<HTMLButtonElement> = (event) => {
    if (countdown <= 0) return;

    event.preventDefault();

    if (!isActive && event.key === 'Enter') {
      startTimer('keyup');
    }
  };

  const onClick: React.ReactEventHandler = (event) => {
    event.preventDefault();

    countdown <= 0 && onTimeUp();
  };

  return (
    <Button {...other} onClick={onClick} onMouseDown={onMouseDown} onKeyDown={onKeyDown}>
      {isActive ? waitingText(remain) : children}
    </Button>
  );
};

const useCountdown = (countdown: number, tickSize: number, onTimeUp: () => void) => {
  type StateType = {
    isActive: boolean,
    remain: number,
  };

  const DEFAULT_STATE: StateType = {
    remain: 0,
    isActive: false,
  };

  const timer = useRef<null | number>(null);

  const { 0: state, 1: setState } = useState<StateType>(DEFAULT_STATE);

  const startTimer = (eventType: 'mouseup' | 'keyup'): void => {
    const clearTimer = (event?: Event): void => {
      event?.preventDefault();

      timer.current && clearTimeout(timer.current);
      timer.current = null;

      window.removeEventListener(eventType, clearTimer);

      setState(DEFAULT_STATE);
    };

    const tick = (remain: number, delay: number) => {
      timer.current = setTimeout(remain > 1 ? tick : act, delay, remain - 1, tickSize);
      setState({ isActive: true, remain: remain });
    };

    const act = () => {
      clearTimer();
      onTimeUp();
    };

    window.addEventListener(eventType, clearTimer, false);

    tick(countdown || DEFAULT_COUNTDOWN, (tickSize * 0.9) << 0);
  };

  return { ...state, startTimer };
};
