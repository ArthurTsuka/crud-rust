// Definindo a interface para os usuários
interface User {
    id: number;
    name: string;
    email: string;
  }
  
  // src/components/UserList.tsx
  import React from 'react';
  
  interface UserListProps {
    users: User[];
  }
  
  const UserList: React.FC<UserListProps> = ({ users }) => {
    return (
      <ul className="mb-8 space-y-4">
        {users.map((user) => (
          <li key={user.id} className="bg-white p-4 rounded-lg shadow-md">
            <p><strong>Nome:</strong> {user.name}</p>
            <p><strong>Email:</strong> {user.email}</p>
          </li>
        ))}
      </ul>
    );
  };
  
  export default UserList;
  