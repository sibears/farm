import React, { useState, useEffect } from 'react';
import { Container, TextField, Button, Typography } from '@mui/material';
import { useCookies } from 'react-cookie';
import { useNavigate } from 'react-router-dom';
import config from './config';
import { useApi } from './api';

const JsonConfigPage = ({ auth, setAuth }) => {
  const [jsonConfig, setJsonConfig] = useState('');
  const [cookie, setCookie] = useCookies(['creds']);
  const navigate = useNavigate();
  const { fetchWithAuth } = useApi();

  useEffect(() => {
    const fetchConfig = async () => {
      if (!auth) {
        navigate('/auth');
        return;
      }

      try {
        const response = await fetchWithAuth(`http://${config.api_url}/api/get_config`, {}, cookie);
        const data = await response.json(); // Извлекаем JSON из результата запроса
        setJsonConfig(JSON.stringify(data, null, 2));
      } catch (error) {
        console.error('Error fetching config:', error);
      }
    };

    fetchConfig();
  }, [auth, cookie, navigate]);

  const handleJsonChange = (e) => {
    setJsonConfig(e.target.value);
  };

  const handleSave = async () => {
    if (!auth) {
      navigate('/auth');
      return;
    }

    try {
      const parsedJson = JSON.parse(jsonConfig);
      const requestOptions = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(parsedJson),
      };
      await fetchWithAuth(`http://${config.api_url}/api/update_config`, requestOptions, cookie);
      alert('Config updated successfully');
    } catch (error) {
      console.error('Error updating config:', error);
      alert('Failed to update config');
    }
  };

  return (
    <Container>
      <Typography variant="h4" gutterBottom>JSON Configuration</Typography>
      <TextField
        label="JSON Config"
        value={jsonConfig}
        onChange={handleJsonChange}
        fullWidth
        multiline
        rows={20}
        margin="normal"
      />
      <Button variant="contained" color="primary" onClick={handleSave} style={{ marginTop: 16 }}>
        Save
      </Button>
    </Container>
  );
};

export default JsonConfigPage;