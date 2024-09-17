// src/App.js
import React, { useEffect, useState } from 'react';
import { getAllProducts, createProduct } from './api';

function App() {
  const [products, setProducts] = useState([]);
  const [newProduct, setNewProduct] = useState({
    name: '',
    category: '',
    price: 0,
    description: '',
    in_stock: false,
    weight: 0,
    discount: null
  });

  useEffect(() => {
    async function fetchProducts() {
      try {
        const data = await getAllProducts();
        setProducts(data);
      } catch (error) {
        console.error("Erro ao carregar produtos:", error);
      }
    }
    fetchProducts();
  }, []);

  const handleCreateProduct = async (e) => {
    e.preventDefault();
    try {
      const product = await createProduct(newProduct);
      setProducts([...products, product]);
      setNewProduct({
        name: '',
        category: '',
        price: 0,
        description: '',
        in_stock: false,
        weight: 0,
        discount: null
      });
    } catch (error) {
      console.error("Erro ao criar produto:", error);
    }
  };

  return (
    <div className="App">
      <h1>Lista de Produtos</h1>
      <ul>
        {products.map(product => (
          <li key={product.id}>
            {product.name} - ${product.price} - {product.category} - {product.description} - 
            {product.in_stock ? 'In Stock' : 'Out of Stock'} - Weight: {product.weight}g - 
            Discount: {product.discount ? product.discount + "%" : "No Discount"}
          </li>
        ))}
      </ul>

      <h2>Criar Novo Produto</h2>
      <form onSubmit={handleCreateProduct}>
        <input
          type="text"
          value={newProduct.name}
          onChange={e => setNewProduct({ ...newProduct, name: e.target.value })}
          placeholder="Nome do Produto"
          required
        />
        <input
          type="text"
          value={newProduct.category}
          onChange={e => setNewProduct({ ...newProduct, category: e.target.value })}
          placeholder="Categoria"
          required
        />
        <input
          type="number"
          value={newProduct.price}
          onChange={e => setNewProduct({ ...newProduct, price: Number(e.target.value) })}
          placeholder="Preço"
          required
        />
        <textarea
          value={newProduct.description}
          onChange={e => setNewProduct({ ...newProduct, description: e.target.value })}
          placeholder="Descrição"
          required
        />
        <label>
          Em Estoque:
          <input
            type="checkbox"
            checked={newProduct.in_stock}
            onChange={e => setNewProduct({ ...newProduct, in_stock: e.target.checked })}
          />
        </label>
        <input
          type="number"
          value={newProduct.weight}
          onChange={e => setNewProduct({ ...newProduct, weight: Number(e.target.value) })}
          placeholder="Peso (g)"
          required
        />
        <input
          type="number"
          value={newProduct.discount ?? ''}
          onChange={e => setNewProduct({ ...newProduct, discount: e.target.value ? Number(e.target.value) : null })}
          placeholder="Desconto (%)"
        />
        <button type="submit">Criar Produto</button>
      </form>
    </div>
  );
}

export default App;
