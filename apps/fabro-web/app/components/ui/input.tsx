import * as React from "react";
import { cn } from "../../lib/utils";

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "flex h-8 w-full border border-line bg-navy-950 px-3 py-1 text-sm text-fg placeholder:text-fg-muted",
          "focus:outline-none focus:border-teal-500 focus:ring-0",
          "disabled:cursor-not-allowed disabled:opacity-50",
          "font-sans",
          className,
        )}
        ref={ref}
        {...props}
      />
    );
  },
);
Input.displayName = "Input";

export { Input };
