import React from 'react'
import ReactDOM from 'react-dom/client'
import { Terminal } from './terminal/Terminal'
import './index.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Terminal />
  </React.StrictMode>,
)
