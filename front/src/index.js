import * as React from 'react';
import * as ReactDOM from 'react-dom/client';
import CssBaseline from '@mui/material/CssBaseline';
import { ThemeProvider } from '@mui/material/styles';
import Controller from './Controller';
import theme from './theme';
import { StyledEngineProvider } from '@mui/material/styles';
import { CookiesProvider } from 'react-cookie';


const rootElement = document.getElementById('root');
const root = ReactDOM.createRoot(rootElement);

root.render(
  <React.StrictMode>
    <StyledEngineProvider injectFirst>
    <CookiesProvider>
      <Controller />
    </CookiesProvider>
    </StyledEngineProvider>
  </React.StrictMode>
);
