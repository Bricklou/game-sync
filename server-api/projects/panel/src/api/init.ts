import { Axios } from "axios";

const api = new Axios({
  baseURL: "/api",
  timeout: 1000,
  withCredentials: true,
});

export default api;
