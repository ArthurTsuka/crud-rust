// src/components/ProductList.tsx
import React from 'react';
import confetti from 'canvas-confetti';

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

interface ProductListProps {
  products: Product[];
}

const ProductList: React.FC<ProductListProps> = ({ products }) => {
  return (
    <ul className="mb-8 space-y-4">
      {products.map((product) => (
        <li key={product.id} className="bg-white p-4 rounded-lg shadow-md">
          <p><strong>Nome:</strong> {product.name}</p>
          <p><strong>Preço:</strong> ${product.price}</p>
          <p><strong>Categoria:</strong> {product.category}</p>
          <p><strong>Descrição:</strong> {product.description}</p>
          <p><strong>Status:</strong> {product.in_stock ? 'In Stock' : 'Out of Stock'}</p>
          <p><strong>Peso:</strong> {product.weight}g</p>
          <p><strong>Desconto:</strong> {product.discount ? `${product.discount}%` : 'No Discount'}</p>
        </li>
      ))}
    </ul>
  );
};

export default ProductList;
