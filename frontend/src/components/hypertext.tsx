import HyperText from "@/components/magicui/hyper-text";

export function HyperTextDemo() {
  return (
    <div className="text-center">
      <HyperText
        className="text-5xl font-bold text-white dark:text-white"
        text="Welcome to the Game"
      />
      <HyperText
        className="text-sm font-medium text-gray-300 mt-4"
        text="Create by Tuti"
      />
    </div>
  );
}
