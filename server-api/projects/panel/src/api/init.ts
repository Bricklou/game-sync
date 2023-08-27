import { useProgressStore } from "./../store/modules/progress";
import { Axios } from "axios";

const api = new Axios({
  baseURL: "/api",
  timeout: 1000,
  withCredentials: true,
});

api.interceptors.request.use((config) => {
  const progressStore = useProgressStore();
  progressStore.show();
  return config;
});

api.interceptors.response.use((response) => {
  const progressStore = useProgressStore();
  progressStore.hide();
  return response;
});

export default api;
