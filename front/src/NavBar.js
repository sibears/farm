import React from 'react';
import { AppBar, Toolbar, Typography, Button, Container } from '@mui/material';
import { Link } from 'react-router-dom';
import FlagCircleIcon from '@mui/icons-material/FlagCircle';
import ThemeSwitcherComponent from './ThemeSwitcher';
import Box from '@mui/material/Box';

const NavBar = ({ prefersDarkMode, changeTheme }) => {
  const [anchorElNav, setAnchorElNav] = React.useState(null);
  const [anchorElUser, setAnchorElUser] = React.useState(null);

  const handleOpenNavMenu = (event) => {
    setAnchorElNav(event.currentTarget);
  };
  const handleOpenUserMenu = (event) => {
    setAnchorElUser(event.currentTarget);
  };

  const handleCloseNavMenu = () => {
    setAnchorElNav(null);
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
          {/* <Button color="inherit" component={Link} to="/config">Config</Button>
          <Button color="inherit" component={Link} to="/json-config">JSON Config</Button> */}
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