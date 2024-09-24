import React from "react";
import { motion } from "framer-motion";

const press = [
  "TheNewYorkTimes",
  "Forbes",
  "Bloomberg",
  "BusinessInsider",
  "TechCrunch",
  "TheGuardian",
  "Wired",
]; 

export function Press() {
  return (
    <section id="press" className="w-full bg-customGreen2 overflow-hidden">
      <div className="py-14 w-full">
        <div className="w-full px-4 md:px-8">
          <h3 className="text-center text-sm font-semibold text-gray-500">
            FEATURED IN
          </h3>
          <div className="relative mt-6 w-full overflow-hidden">
            <motion.div
              className="flex space-x-8 items-center justify-start w-max"
              animate={{ x: ["0%", "-50%"] }} 
              transition={{
                repeat: Infinity,
                duration: 120, 
                ease: "linear",
              }}
            >
              {[...press, ...press].map((logo, idx) => (
                <img
                  key={idx}
                  src={`https://cdn.magicui.design/press/${logo}.svg`}
                  className="h-10 w-40 px-2 dark:brightness-0 dark:invert"
                  alt={`logo-${logo}`}
                />
              ))}
            </motion.div>
          </div>
        </div>
      </div>
    </section>
  );
}
