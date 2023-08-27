import api from "@/api/init";
import { IUser } from "@/types/user.interface";

export async function login(email: string, password: string): Promise<IUser> {
  const response = await api.post<IUser>("/auth/login", {
    email,
    password,
  });

  return response.data;
}

export async function logout(): Promise<void> {
  await api.post("/auth/logout");
}

export async function me(): Promise<IUser> {
  const response = await api.get<IUser>("/auth/me");

  return response.data;
}

export async function register(
  email: string,
  password: string,
): Promise<IUser> {
  const response = await api.post<IUser>("/auth/register", {
    email,
    password,
  });

  return response.data;
}
