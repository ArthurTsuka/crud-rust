import ShimmerButton from "@/components/magicui/shimmer-button";

export function ShimmerButtonDemo() {
  return (
    <div className="z-4 flex min-h-[rem] items-center justify-center">
      <ShimmerButton className="shadow-2xl px-4 py-2">
        <span className="whitespace-pre-wrap text-center text-xs font-semibold leading-none tracking-tight text-white dark:from-white dark:to-slate-900/10 lg:text-lg">
          Try for Free
        </span>
      </ShimmerButton>
    </div>
  );
}
