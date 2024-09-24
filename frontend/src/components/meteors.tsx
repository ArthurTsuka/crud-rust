import Meteors from "@/components/magicui/meteors";
import { HyperTextDemo } from '@/components/hypertext';
import { HeroVideoDialogDemo } from "./heroVideo";

export function MeteorDemo() {
  return (
    <div className="relative flex h-[500px] w-full flex-col items-center justify-center overflow-hidden rounded-lg bg-customDark md:shadow-xl">
      <Meteors number={35} />
      <HyperTextDemo />
    </div>
  );
}
