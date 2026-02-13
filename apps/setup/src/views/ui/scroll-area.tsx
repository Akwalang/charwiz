import * as React from "react";
import * as ScrollAreaPrimitive from "@radix-ui/react-scroll-area";

import { cn } from "@/utils/react";

const ScrollArea = React.forwardRef<
  React.ElementRef<typeof ScrollAreaPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof ScrollAreaPrimitive.Root>
>(({ className, children, ...props }, ref) => (
  <ScrollAreaPrimitive.Root
    ref={ref}
    className={cn("relative overflow-hidden", className)}
    {...props}
  >
    <ScrollAreaPrimitive.Viewport className="h-full w-full rounded-[inherit]">
      {children}
    </ScrollAreaPrimitive.Viewport>
    <ScrollBar />
    <ScrollAreaPrimitive.Corner />
  </ScrollAreaPrimitive.Root>
));
ScrollArea.displayName = ScrollAreaPrimitive.Root.displayName;

const ScrollBar = React.forwardRef<
  React.ElementRef<typeof ScrollAreaPrimitive.ScrollAreaScrollbar>,
  React.ComponentPropsWithoutRef<typeof ScrollAreaPrimitive.ScrollAreaScrollbar>
>(({ className, orientation = "vertical", ...props }, ref) => (
  <ScrollAreaPrimitive.ScrollAreaScrollbar
    ref={ref}
    orientation={orientation}
    className={cn(
      "flex touch-none select-none transition-colors",
      orientation === "vertical" && "h-full w-2.5  p-[1px]",
      orientation === "horizontal" && "h-2.5 flex-col p-[1px]",
      className
    )}
    {...props}
  >
    <ScrollAreaPrimitive.ScrollAreaThumb className="relative flex-1 rounded-full bg-border" />
  </ScrollAreaPrimitive.ScrollAreaScrollbar>
));
ScrollBar.displayName = ScrollAreaPrimitive.ScrollAreaScrollbar.displayName;

type HorizontalScrollAreaProps = React.ComponentPropsWithoutRef<typeof ScrollArea>;

export const HorizontalScrollArea = ({ children, ...props }: HorizontalScrollAreaProps) => {
  const scroll = React.useRef<number>(0);
  const ref = React.useRef<HTMLDivElement>(null)

  React.useEffect(() => {
    const el = ref.current;

    if (!el) return;

    const scrollBox = el.querySelector('& > div')!;

    const onWheel = (e: WheelEvent) => {
      if (e.deltaY === 0) return;

      e.preventDefault();

      const min = 0;
      const max = scrollBox.scrollWidth - scrollBox.clientWidth;

      let offset = scroll.current + e.deltaY;

      offset < min && (offset = min);
      offset > max && (offset = max);

      scrollBox.scrollTo({
        left: offset,
        behavior: 'smooth',
      });

      scroll.current = offset;
    };

    el.addEventListener('wheel', onWheel, { passive: false });

    return () => el.removeEventListener('wheel', onWheel);
  }, [])

  return (
    <ScrollArea
      ref={ref}
      {...props}
      className={`whitespace-nowrap ${props.className || ''}`}
    >
      {children}
    </ScrollArea>
  );
}

export { ScrollArea, ScrollBar };
