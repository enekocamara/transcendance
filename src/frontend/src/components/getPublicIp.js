import axios from 'axios';

async function getPublicIp() {
  const response = await axios.get('https://httpbin.org/ip');
  return response.data.origin;
}

export default getPublicIp;
