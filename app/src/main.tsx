import React from 'react'
import ReactDOM from 'react-dom/client'
import { ChakraProvider } from '@chakra-ui/react'
import App from './App'
import theme from '../helpers/theme'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ChakraProvider theme={{ ...theme, colors: { ...theme.colors, primary: theme.colors['red'] } }}>
      <App />
    </ChakraProvider>
  </React.StrictMode>
)

