import React, { useState, useEffect } from 'react';
import { Box, Container, Grid, Typography } from "@mui/material";
import { useCookies } from 'react-cookie';
import config from './config';
import { BarChart } from '@mui/x-charts/BarChart';
import MuiAccordion from '@mui/material/Accordion';
import MuiAccordionSummary from '@mui/material/AccordionSummary';
import MuiAccordionDetails from '@mui/material/AccordionDetails';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import ReportProblemIcon from '@mui/icons-material/ReportProblem';
import InfoIcon from '@mui/icons-material/Info';
import AutorenewIcon from '@mui/icons-material/Autorenew';
import DoneIcon from '@mui/icons-material/Done';
import { useNavigate } from 'react-router-dom';
import { useApi } from './api';


const Statistic = ({ auth, setAuth }) => {
    const [flagsData, setFlagsData] = useState({});
    const [cookie, setCookie] = useCookies(['creds']);
    const navigate = useNavigate();
    const { fetchWithAuth } = useApi();
  
    useEffect(() => {
      const fetchData = async () => {
        if (!auth) {
          navigate('/auth');
          return;
        }
  
        try {
          const res = await fetchWithAuth(`http://${config.api_url}/api/flags/stats`, {}, cookie);
          const data = await res.json();
  
          // Если data уже массив, оставляем его, иначе преобразуем в массив пар [ключ, значение]
          const statsArray = Array.isArray(data) ? data : Object.entries(data);
          setFlagsData(statsArray);
        } catch (error) {
          console.error("Error fetching flags data:", error);
        }
      };
  
      fetchData();
    }, [auth, cookie, navigate]);
  
    // Преобразуем flagsData в массив, если вдруг он не массив
    const statsArray = Array.isArray(flagsData) ? flagsData : Object.entries(flagsData);
    const chartData = statsArray.map(([status, count]) => ({
      id: status,
      label: status,
      value: count,
    }));
  
    chartData.sort((a, b) => a.id.localeCompare(b.id));
    console.log(chartData);

    return (
      <Grid container spacing={2}>
          <Grid item xs={12} md={8} lg={9} order={{ xs: 2, md: 1 }}>
              <Container sx={{ flexWrap: 'nowrap' }}>
                  <Box component="div" p={5}>
                      {chartData && chartData.length > 0 && (
                          <BarChart 
                              sx={{
                                  "& .MuiBarElement-root:nth-child(1)": {
                                      fill: "#388e3c", // Accepted
                                  },
                                  "& .MuiBarElement-root:nth-child(2)": {
                                      fill: "#f57c00", // Queued
                                  }, 
                                  "& .MuiBarElement-root:nth-child(3)": {
                                      fill: "#d32f2f", // Rejected
                                  },
                                  "& .MuiBarElement-root:nth-child(4)": {
                                      fill: "#0288d1", // Skipped
                                  },
                              }}
                              xAxis={[
                                  {
                                      id: "Flags Stats",
                                      data: chartData.map(item => item.label),
                                      scaleType: 'band',
                                  },
                              ]} 
                              series={[
                                  {
                                      data: chartData.map(item => item.value),
                                  }
                              ]}
                              width={800}
                              height={400}
                          />
                      )}
                  </Box>
              </Container>
          </Grid>
            <Grid item xs={12} md={4} lg={3} order={{ xs: 1, md: 2 }}>
                <Container sx={{ flexWrap: 'nowrap' }}>
                    <Box p={2}>
                        <Typography variant="h6" gutterBottom>
                            More info about flag status
                        </Typography>
                        {[{
                            icon: <DoneIcon />,
                            title: "Accepted",
                            details: "The checksystem has considered the flag correct."
                        }, {
                            icon: <AutorenewIcon />,
                            title: "Queued",
                            details: "The flag is in the queue for submitting to the checksystem. This means that either the server didn't send the flag yet, or it didn't get a clear response (e.g. the checksystem was down or reported that the quota was exceeded). In the latter case, the 'Checksystem Response' field would be non-empty, and the flag will be resent soon."
                        }, {
                            icon: <ReportProblemIcon />,
                            title: "Rejected",
                            details: "The checksystem has considered the flag incorrect."
                        }, {
                            icon: <InfoIcon />,
                            title: "Skipped",
                            details: "The farm didn't get a clear response regarding the flag during its lifetime. It doesn't make sense to resent the flag anymore, so it was excluded from the submission queue."
                        }].map((accordion, index) => (
                            <MuiAccordion key={index}>
                                <MuiAccordionSummary
                                    expandIcon={<ExpandMoreIcon />}
                                    aria-controls={`panel${index + 1}a-content`}
                                    id={`panel${index + 1}a-header`}
                                >
                                    <Box sx={{ display: 'flex', alignItems: 'center' }}>
                                        {accordion.icon}
                                        <Typography sx={{ ml: 1 }}>{accordion.title}</Typography>
                                    </Box>
                                </MuiAccordionSummary>
                                <MuiAccordionDetails>
                                    <Typography>
                                        {accordion.details}
                                    </Typography>
                                </MuiAccordionDetails>
                            </MuiAccordion>
                        ))}
                    </Box>
                </Container>
            </Grid>
      </Grid>
  );
};


export default Statistic