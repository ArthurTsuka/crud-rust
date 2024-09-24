// src/App.tsx
import React, { useEffect, useState } from 'react';
import Navbar from '@/components/navbar';
import { Press } from '@/components/socialProof';
import { MeteorDemo } from '@/components/meteors';
import { OrbitingCirclesDemo } from '@/components/orbitingGlobe';

interface Product {
  id: number;
  name: string;
  category: string;
  price: number;
  description: string;
  in_stock: boolean;
  weight: number;
  discount?: number | null;
}

interface User {
  id: number;
  name: string;
  email: string;
}

function App() {

  return (
    <div className="min-h-screen bg-customDark">
      <Navbar />
      <MeteorDemo />
      <div>
        <Press />
      </div>
    </div>
  );
}

export default App;
