import { DataGrid } from '@mui/x-data-grid'
import React, { useState, useEffect, useRef} from 'react'
import { renderStatus } from './Status';
import { minWidth } from '@mui/system';
import { useCookies } from 'react-cookie';
import config from "./config";
import { useApi } from './api';
import { useNavigate } from 'react-router-dom';
import useInterval from './useInterval';



const columns = [
  { field: 'id', headerName: 'ID', width: 80},
  { field: 'flag', headerName: 'Flag', minWidth: 380, flex: 1 },
  { field: 'status', headerName: 'Status', type: "singleSelect", valueOptions: ["ACCEPTED", "REJECTED", "SKIPPED", "QUEUED"], renderCell: renderStatus, minWidth: 150, flex: 1 },
  { field: 'sploit', headerName: 'Sploit', minWidth: 150, flex: 1, },
  { field: 'team', headerName: 'Team', minWidth: 100, flex: 1 },
  { field: 'created_time', headerName: 'Time', minWidth: 100, flex: 1},
//   { field: 'start_waiting_time', headerName: 'Start Waiting Time', minWidth: 100, flex: 1},
  { field: 'checksystem_response', headerName: "Response", minWidth: 600, flex: 1 }
]

const FlagsList = ({ auth, setAuth }) => {
    const [flagsData, setFlagsData] = useState([]);
    const [totalRows, setTotalRows] = useState(0);
    const [paginationModel, setPaginationModel] = useState({
        pageSize: 20,
        page: 0,
    });
    const [loading, setLoading] = useState(false);
    const [cookie, setCookie] = useCookies(['creds']);
    const { fetchWithAuth } = useApi();
    const navigate = useNavigate();
    
    const fetchFlags = async (page, pageSize) => {
        if (!auth) {
            navigate('/auth');
            return;
        }

        setLoading(true);
        try {
            const offset = page * pageSize;
            
            // Получить флаги для текущей страницы
            const flagsResponse = await fetchWithAuth(
              `http://${config.api_url}/api/flags?limit=${pageSize}&offset=${offset}`,
              {},
              cookie
            );
            if (!flagsResponse.ok) {
              throw new Error(
                `Error fetching flags: ${flagsResponse.status} ${flagsResponse.statusText}`
              );
            }
            const flags = await flagsResponse.json();
            setFlagsData(flags);
        
            // Получить общее число флагов
            const totalResponse = await fetchWithAuth(
              `http://${config.api_url}/api/flags/total`,
              {},
              cookie
            );
            if (!totalResponse.ok) {
              throw new Error(
                `Error fetching total flags count: ${totalResponse.status} ${totalResponse.statusText}`
              );
            }
            // Предполагается, что /api/flags/total возвращает объект вида { total: число }
            const totalData = await totalResponse.json();
            setTotalRows(totalData);
          } catch (error) {
            console.error("Error fetching flags data:", error);
          } finally {
            setLoading(false);
          }
    };

    useEffect(() => {
        fetchFlags(paginationModel.page, paginationModel.pageSize);
    }, [paginationModel, auth]);

    // useInterval(async () => {
    //     if (loading) return;
    //     fetchFlags(paginationModel.page, paginationModel.pageSize);
    // }, 5000);

    return (
        <DataGrid
            rows={flagsData}
            rowCount={totalRows ?? 0} // если totalRows undefined, то используется 0
            columns={columns}
            loading={loading}
            pageSizeOptions={[10, 20, 50]}
            paginationModel={paginationModel}
            paginationMode="server"
            onPaginationModelChange={setPaginationModel}
            disableRowSelectionOnClick
            initialState={{
                sorting: {
                    sortModel: [{ field: 'id', sort: 'desc' }],
                },
            }}
        />
    );
};

export default FlagsList