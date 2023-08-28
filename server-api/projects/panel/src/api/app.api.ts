import { IAppStatus } from "./../types/app.interface";
import api from "@/api/init";

export async function status(): Promise<IAppStatus> {
  const response = await api.get<IAppStatus>("/");

  return response.data;
}
