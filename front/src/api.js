import { useNavigate } from 'react-router-dom';
import { useCookies } from 'react-cookie';
import config from './config';

export const useApi = () => {
  const navigate = useNavigate();
  const [cookie, setCookie] = useCookies(['creds']);

  const fetchWithAuth = async (url, options = {}) => {
    if (!cookie.creds) {
      navigate('/auth');
      return;
    }

    const requestOptions = {
      ...options,
      headers: {
        ...options.headers,
        'X-Authorization': cookie.creds,
      },
    };

    const response = await fetch(url, requestOptions);

    if (response.status === 400) {
      navigate('/auth');
      return;
    }

    return response;
  };

  return { fetchWithAuth };
};