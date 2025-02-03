import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Container from "@mui/material/Container";
import React, { useEffect, useState } from "react";
import { Alert } from "@mui/material";
import { useCookies } from 'react-cookie'
import { useNavigate } from 'react-router-dom';
import config from "./config";


export default function Auth(props) {
  let [errorMsg, setErrorMsg] = useState("");
  const [cookies, setCookie] = useCookies(['creds'])
  const navigate = useNavigate();

  const handleSubmit = (event) => {
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    const password = data.get('password')
    const requestOptions = {
      method: 'POST',
      headers: { 
          'Content-Type': 'application/json',
      },
      body: JSON.stringify({passwd: data.get('password')})
    };
    fetch(`http://${config.api_url}/api/check_auth`, requestOptions)
      .then((data) => data.json())
      .then((data) => {
        if (data == "ok") {
          setCookie("creds", password.toString(), { path: "/", secure: false, httpOnly: false} )
          props.setAuth(true)
          navigate('/');
        } else {
          props.setAuth(false)
          setErrorMsg("Invalid password")
        }
      })
      .catch(err => {
        setErrorMsg(err.toString())
      })
  };

  return (
    <Container component="main" maxWidth="xs">
      <Box
        sx={{  
          marginTop: 8,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <Typography component="h1" variant="h5">
          Enter password
        </Typography>
        <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
          <TextField
            margin="normal"
            required
            fullWidth
            name="password"
            label="Password"
            type="password"
            id="password"
            autoComplete="current-password"
          />
          <Button
            type="submit"
            fullWidth
            variant="contained"
            sx={{ mt: 3, mb: 2 }}
          >
            Submit
          </Button>
        </Box>
        { errorMsg !== "" ?
        <Alert severity="error">{ errorMsg }</Alert>
        :
        <div></div>
      }
      </Box>
    </Container>
  );
}