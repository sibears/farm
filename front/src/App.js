import React from "react";
import FlagsList from "./FlagsList";
import Auth from "./Auth";
import config from "./config";
import {
  Box,
  Container,
} from "@mui/material";

const App = ({ auth, setAuth }) => {

    return (
        <div>
                <div>
                <Container maxWidth="400">
                    <Box component="div" p={5}></Box>
                    <FlagsList auth={auth} setAuth={setAuth} />
                </Container>
                </div>
        </div>
    )
}

export default App;