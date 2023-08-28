import { useProgressStore } from "./../store/modules/progress";
import axios from "axios";

const api = axios.create({
  baseURL: "/api",
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
