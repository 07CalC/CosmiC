import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './global.css'
import App from './App.tsx'
import { SetupProvider } from './context/SetupContext.tsx'
import { GlobalProvider } from './context/GlobalContext.tsx'
import { AuthProvider } from './context/authContext.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <GlobalProvider>
    <SetupProvider>
      <AuthProvider>
    <App />
    </AuthProvider>
    </SetupProvider>
    </GlobalProvider>
  </StrictMode>,
)
