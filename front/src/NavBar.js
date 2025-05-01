import React from 'react';
import { AppBar, Toolbar, Typography, Button, Container } from '@mui/material';
import { Link } from 'react-router-dom';
import FlagCircleIcon from '@mui/icons-material/FlagCircle';
import ThemeSwitcherComponent from './ThemeSwitcher';
import Box from '@mui/material/Box';
import config from "./config";

const NavBar = ({ prefersDarkMode, changeTheme }) => {
  const handleDownload = () => {
    window.location.href = `http://${config.api_url}/api/start_sploit.py`;
  };

  return (
    <AppBar position="static">
      <Container maxWidth="xl">
        <Toolbar disableGutters>
          <FlagCircleIcon sx={{ display: { xs: 'none', md: 'flex' }, mr: 1 }} />
          <Typography
            variant="overline"
            noWrap
            component="a"
            href="/"
            sx={{
              mr: 2,
              display: { xs: 'none', md: 'flex' },
              fontFamily: 'sans-serif',
              fontWeight: 700,
              letterSpacing: '.1rem',
              color: 'inherit',
              textDecoration: 'none',
            }}
          >
            SiBears Farm
          </Typography>
          <Button color="inherit" component={Link} to="/">Home</Button>
          <Button color="inherit" component={Link} to="/stat">Statistics</Button>
          <Button color="inherit" onClick={handleDownload}>Start Sploit</Button>
          <Box sx={{ flexGrow: 1 }} />
          <Box sx={{ flexGrow: 0 }}>
            <ThemeSwitcherComponent useDark={prefersDarkMode} themeChanger={changeTheme} />
          </Box>
        </Toolbar>
      </Container>
    </AppBar>
  );
};

export default NavBar;
