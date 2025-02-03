import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Container, TextField, Button, Typography, Grid, Paper, IconButton } from '@mui/material';
import DeleteIcon from '@mui/icons-material/Delete';
import config from './config';
import { useApi } from './api';

const ConfigPage = ({ auth, setAuth }) => {
  const navigate = useNavigate();
  const [configData, setConfigData] = useState(null);
  const [editableConfig, setEditableConfig] = useState(null);
  const [newTeam, setNewTeam] = useState({ key: '', value: '' });
  const { fetchWithAuth } = useApi();

  useEffect(() => {
    const fetchConfig = async () => {
      const response = await fetchWithAuth(`http://${config.api_url}/api/get_config`);
      if (response) {
        const data = await response.json();
        setConfigData(data);
        setEditableConfig(data);
      }
    };

    fetchConfig();
  }, [fetchWithAuth]);

  const handleChange = (section, key, value) => {
    setEditableConfig({
      ...editableConfig,
      [section]: {
        ...editableConfig[section],
        [key]: value,
      },
    });
  };

  const handleNestedChange = (section, subsection, key, value) => {
    setEditableConfig({
      ...editableConfig,
      [section]: {
        ...editableConfig[section],
        [subsection]: {
          ...editableConfig[section][subsection],
          [key]: value,
        },
      },
    });
  };

  const handleAddTeam = () => {
    setEditableConfig({
      ...editableConfig,
      ctf: {
        ...editableConfig.ctf,
        teams: {
          ...editableConfig.ctf.teams,
          [newTeam.key]: newTeam.value,
        },
      },
    });
    setNewTeam({ key: '', value: '' });
  };

  const handleDeleteTeam = (teamKey) => {
    const updatedTeams = { ...editableConfig.ctf.teams };
    delete updatedTeams[teamKey];
    setEditableConfig({
      ...editableConfig,
      ctf: {
        ...editableConfig.ctf,
        teams: updatedTeams,
      },
    });
  };

  const handleSave = async () => {
    const requestOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(editableConfig),
    };
    const response = await fetchWithAuth(`http://${config.api_url}/update_config`, requestOptions);

    if (response && response.ok) {
      setConfigData(editableConfig);
      alert('Config updated successfully');
    } else {
      alert('Failed to update config');
    }
  };

  if (!configData) {
    return <Typography>Loading...</Typography>;
  }

  const sortedTeams = Object.entries(editableConfig.ctf.teams).sort((a, b) => {
    const ipA = a[1].split('.').map(Number);
    const ipB = b[1].split('.').map(Number);
    for (let i = 0; i < 4; i++) {
      if (ipA[i] < ipB[i]) return -1;
      if (ipA[i] > ipB[i]) return 1;
    }
    return 0;
  });

  const handleTeamNameChange = (oldKey, newKey) => {
    const updatedTeams = { ...editableConfig.ctf.teams };
    const value = updatedTeams[oldKey];
    delete updatedTeams[oldKey];
    updatedTeams[newKey] = value;
    setEditableConfig({
      ...editableConfig,
      ctf: {
        ...editableConfig.ctf,
        teams: updatedTeams,
      },
    });
  };

  return (
    <Container>
      <Typography variant="h4" gutterBottom>Configuration</Typography>
      <Grid container spacing={3}>
        {Object.keys(configData).map((section) => (
          <Grid item xs={12} key={section}>
            <Paper style={{ padding: 16 }}>
              <Typography variant="h6">{section}</Typography>
              {section === 'ctf' ? (
                <>
                  <Typography variant="subtitle1">Protocol</Typography>
                  {Object.keys(configData[section].protocol).map((key) => (
                    <TextField
                      key={key}
                      label={key}
                      value={editableConfig[section].protocol[key]}
                      onChange={(e) => handleNestedChange(section, 'protocol', key, e.target.value)}
                      fullWidth
                      margin="normal"
                    />
                  ))}
                  <Typography variant="subtitle1">Teams</Typography>
                  {sortedTeams.map(([key, value]) => (
                    <Grid container spacing={2} alignItems="center" key={key}>
                      <Grid item xs={5}>
                        <TextField
                          label="Team Name"
                          value={key}
                          onChange={(e) => handleTeamNameChange(key, e.target.value)}
                          fullWidth
                          margin="normal"
                        />
                      </Grid>
                      <Grid item xs={5}>
                        <TextField
                          label="Team IP"
                          value={value}
                          onChange={(e) => handleNestedChange('ctf', 'teams', key, e.target.value)}
                          fullWidth
                          margin="normal"
                        />
                      </Grid>
                      <Grid item xs={2}>
                        <IconButton
                          color="secondary"
                          onClick={() => handleDeleteTeam(key)}
                          style={{ marginTop: 16 }}
                        >
                          <DeleteIcon />
                        </IconButton>
                      </Grid>
                    </Grid>
                  ))}
                  <Grid container spacing={2} alignItems="center">
                    <Grid item xs={5}>
                      <TextField
                        label="Team Name"
                        value={newTeam.key}
                        onChange={(e) => setNewTeam({ ...newTeam, key: e.target.value })}
                        fullWidth
                        margin="normal"
                      />
                    </Grid>
                    <Grid item xs={5}>
                      <TextField
                        label="Team IP"
                        value={newTeam.value}
                        onChange={(e) => setNewTeam({ ...newTeam, value: e.target.value })}
                        fullWidth
                        margin="normal"
                      />
                    </Grid>
                    <Grid item xs={2}>
                      <Button
                        variant="contained"
                        color="primary"
                        onClick={handleAddTeam}
                        style={{ marginTop: 16 }}
                      >
                        Add Team
                      </Button>
                    </Grid>
                  </Grid>
                </>
              ) : (
                Object.keys(configData[section]).map((key) => (
                  <TextField
                    key={key}
                    label={key}
                    value={editableConfig[section][key]}
                    onChange={(e) => handleChange(section, key, e.target.value)}
                    fullWidth
                    margin="normal"
                  />
                ))
              )}
            </Paper>
          </Grid>
        ))}
      </Grid>
      <Button variant="contained" color="primary" onClick={handleSave} style={{ marginTop: 16 }}>
        Save
      </Button>
    </Container>
  );
};

export default ConfigPage;