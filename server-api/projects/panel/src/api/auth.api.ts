import api from "@/api/init";
import { IUser } from "@/types/user.interface";

export async function login(email: string, password: string): Promise<IUser> {
  const response = await api.post<{ user: IUser }>("/admin/auth", {
    email,
    password,
  });

  return response.data;
}

export async function logout(): Promise<void> {
  await api.delete("/admin/auth");
}

export async function me(): Promise<IUser> {
  const response = await api.get<IUser>("/admin/auth");

  return response.data;
}

export async function register(
  email: string,
  password: string,
): Promise<IUser> {
  const response = await api.post<IUser>(
    "/admin/auth/register",
    {
      email,
      password,
    },
    {
      headers: {
        "Content-Type": "application/json",
      },
    },
  );

  return response.data;
}
