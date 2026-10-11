import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    host: '0.0.0.0',
    port: 5173,
    proxy: {
      '/api': {
        target: process.env.VITE_API_URL || 'http://server:3001',
        changeOrigin: true,
        // Fallback local se rodando fora do docker
        configure: (proxy) => {
          proxy.on('error', (err) => {
            console.log('Proxy error, attempting localhost:3001 fallback:', err.message);
          });
        },
      },
    },
  },
});
