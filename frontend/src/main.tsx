import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import Router from './ui/router'
import { ConfigProvider } from 'antd'
import { theme } from './ui/theme'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <ConfigProvider theme={theme}>
        <Router />
      </ConfigProvider>
    </BrowserRouter>
  </StrictMode >,
)
