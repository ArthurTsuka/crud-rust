// src/api.js
import axios from 'axios';

const API_URL = 'http://localhost:8080';

export const getAllProducts = async () => {
  try {
    const response = await axios.get(`${API_URL}/product`);
    return response.data;
  } catch (error) {
    console.error("Erro ao buscar produtos", error);
    throw error;
  }
};

export const createProduct = async (productData) => {
  try {
    const response = await axios.post(`${API_URL}/product`, productData);
    return response.data;
  } catch (error) {
    console.error("Erro ao criar produto", error);
    throw error;
  }
};
