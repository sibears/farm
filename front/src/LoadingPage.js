import React from 'react';
import { Box, CircularProgress, Typography } from '@mui/material';

const LoadingPage = () => {
  return (
    <Box
      sx={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        bgcolor: theme => theme.palette.background.default,
        color: theme => theme.palette.text.primary,
      }}
    >
      
      <CircularProgress size={80} thickness={4} sx={{ mr: 2 }} />
      
      <Typography variant="h4">
        Загрузка...
      </Typography>
    </Box>
  );
};

export default LoadingPage;