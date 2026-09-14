import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

import { cn } from "@/shared/lib/cn";

const buttonVariants = cva(
  "inline-flex shrink-0 items-center justify-center gap-2 rounded-[var(--radius-control)] border text-[length:var(--font-label)] font-medium tracking-normal transition-colors duration-[var(--duration-fast)] outline-none focus-visible:ring-1 focus-visible:ring-[var(--focus)] focus-visible:ring-offset-1 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        default:
          "border-[var(--border-control)] bg-[var(--surface)] text-[var(--text)] hover:bg-[var(--surface-subtle)]",
        primary:
          "border-[var(--text)] bg-[var(--text)] text-[var(--surface)] hover:bg-[var(--primary-hover)]",
        quiet:
          "border-transparent bg-transparent text-[var(--text)] hover:bg-[var(--surface-hover)]",
      },
      size: {
        default: "h-8 px-3 py-1",
        icon: "size-[var(--button-size)] p-0",
        smallIcon: "size-[var(--button-small-size)] p-0",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

interface ButtonProps
  extends ComponentProps<"button">, VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

export function Button({
  asChild = false,
  className,
  size,
  type = "button",
  variant,
  ...props
}: ButtonProps) {
  const Component = asChild ? Slot : "button";

  return (
    <Component
      className={cn(buttonVariants({ className, size, variant }))}
      type={asChild ? undefined : type}
      {...props}
    />
  );
}
