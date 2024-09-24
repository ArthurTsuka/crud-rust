// src/pages/_app.tsx
import '../styles/global.css'; // Importa o CSS global com Tailwind
import { AppProps } from 'next/app';

function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

export default MyApp;