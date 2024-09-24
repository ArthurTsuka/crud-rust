// src/api/user.ts
import axios from 'axios';

const API_URL = 'http://localhost:8080';

interface User {
  id: number;
  name: string;
  email: string;
}

export const getAllUsers = async (): Promise<User[]> => {
  try {
    const response = await axios.get(`${API_URL}/users`);
    return response.data;
  } catch (error) {
    console.error("Erro ao buscar usuários", error);
    throw error;
  }
};