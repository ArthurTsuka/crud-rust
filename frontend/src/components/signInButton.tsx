"use client";

import React from "react";
import { motion } from "framer-motion";

interface SignInButtonProps {
  onClick: () => void;
  buttonText: string;
}

const SignInButton: React.FC<SignInButtonProps> = ({ onClick, buttonText }) => {
  return (
    <motion.button
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      className="px-5 py-2 bg-black text-white rounded-md shadow-md border font-semibold border-white/10 hover:bg-customGreen transition duration-300 ease-in-out"
      onClick={onClick}
    >
      {buttonText}
    </motion.button>
  );
};

export default SignInButton;
