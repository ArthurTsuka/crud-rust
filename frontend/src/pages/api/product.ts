// src/api/product.ts
import axios from 'axios';

const API_URL = 'http://localhost:8080';

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

export const getAllProducts = async (): Promise<Product[]> => {
  try {
    const response = await axios.get(`${API_URL}/product`);
    return response.data;
  } catch (error) {
    console.error("Erro ao buscar produtos", error);
    throw error;
  }
};

export const createProduct = async (productData: Product): Promise<Product> => {
  try {
    const response = await axios.post(`${API_URL}/product`, productData);
    return response.data;
  } catch (error) {
    console.error("Erro ao criar produto", error);
    throw error;
  }
};
