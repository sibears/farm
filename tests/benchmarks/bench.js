import http from 'k6/http';
import { sleep, check } from 'k6';

let config = JSON.parse(open("../config.json"))

export const options = {
  vus: 10,
  duration: '30s',
};
export default function () {
  const url = "http://localhost:8000"
  const param = JSON.stringify({
    passwd: config['auth']['password']
  });
  let res = http.post(`${url}/api/check_auth`,param)
  check(res, {
    'is correct': (r) => r.body.indexOf("ok") !== -1
  })
  
  const hea = {
    headers: {
      'Authorization': config['auth']['password'],
      'Content-Type': 'application/json'
    }
  }
  let res1 = http.get(`${url}/api/get_config`, hea)
  check(res1, {
    'check config': (r1) => r1.body == JSON.stringify(config)
  })

  http.get(`${url}/api/flag`, hea)
}
