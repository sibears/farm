import { useState } from "react";
import React from "react";
import App from "./App"
import Statistic from "./Statistic";

import {
  CssBaseline,
} from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import themes from "./theme";
import useMediaQuery from "@mui/material/useMediaQuery";
import NavBar from "./NavBar";
import { Route, Routes, BrowserRouter as Router} from "react-router-dom";
import { useCookies } from 'react-cookie'
import { useEffect } from "react";
import config from "./config";
import LoadingPage from "./LoadingPage";
import ConfigPage from "./ConfigPage";
import JsonConfigPage from "./JsonConfigPage";
import Auth from "./Auth";



const Controller = () => {
  const [isDarkTheme, setIsDarkTheme] = useState(() => {
    // Получаем значение из локального хранилища
    const localData = localStorage.getItem('isDarkTheme');
    // Если в локальном хранилище значение есть, то возвращаем его
    if (localData) {
      return JSON.parse(localData);
    }
    // Иначе возвращаем false
    return false;
  })
  const [isLoading, setIsLoading] = useState(true);
  const [auth, setAuth] = useState(false);
  const [cookies, setCookie] = useCookies(['creds'])

  useEffect(() => {
    const checkAuth = async (passwd) => {
      const requestOptions = {
          method: 'POST',
          headers: {
          'Content-Type': 'application/json',
          },
          body: JSON.stringify({ passwd: passwd })
      };
      const res = await fetch(`http://${config.api_url}/api/check_auth`, requestOptions)
      const data = await res.json()
      if (data === "ok") {
          setAuth(true)
      } else {
          setAuth(false)
      }
      setIsLoading(false);
    };
    if (cookies.creds) {
        checkAuth(cookies.creds)
    } else {
        setIsLoading(false)
    }
  }, [cookies.creds])

  const changeTheme = (isDarkMode) => {
    setIsDarkTheme(isDarkMode)
    // Сохраняем тип темы в локальное хранилище
    localStorage.setItem('isDarkTheme', JSON.stringify(isDarkMode));
  }

  return (
    <ThemeProvider theme={isDarkTheme ? themes.dark : themes.light}>
        <Router>
          <NavBar prefersDarkMode={isDarkTheme} changeTheme={changeTheme} />
          <CssBaseline />
          { isLoading ? (
            <LoadingPage /> // Используем компонент загрузки
          ) : (
            <Routes>
              <Route path="/" exact element={<App  auth={auth} setAuth={setAuth} />} />
              <Route path="/stat" element={<Statistic auth={auth} setAuth={setAuth}/>} />
              {/* <Route path="/config" element={<ConfigPage auth={auth} setAuth={setAuth}/>} />
              <Route path="/json-config" element={<JsonConfigPage auth={auth} setAuth={setAuth} />} /> */}
              <Route path="/auth" element={<Auth setAuth={setAuth} />} />
            </Routes>
            )
          }
        </Router>
    </ThemeProvider>

  )
}

export default Controller