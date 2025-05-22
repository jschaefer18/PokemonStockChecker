
// This file is the entry point of the React app. It initializes the application and renders the App component into the DOM. 
// It also applies global styles from index.css and uses React’s StrictMode to help catch bugs during development.

import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
