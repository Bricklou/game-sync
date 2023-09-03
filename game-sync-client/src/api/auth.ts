import { useAppStore } from "@/store/modules/app";
import { User } from "@/types/user";
import { store } from "./store";

export async function login(email: string, password: string): Promise<User> {
  const appStore = useAppStore();

  // Send the request
  let response: Response;
  try {
    response = await appStore.fetch("/api/auth", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email,
        password,
      }),
    });
  } catch (e) {
    throw new Error("Failed to login: " + e);
  }

  // Check if the credentials are valid
  if (response.status == 401) {
    throw new Error("Invalid credentials");
  }

  // Check if the request was successful
  if (!response.ok) {
    console.error(response);
    throw new Error("Failed to login: status code " + response.status);
  }

  // Extract the token from the response
  let token = response.headers.get("WWW-Authenticate");
  if (!token) {
    throw new Error("Failed to login: no token");
  }
  token = token.replace("Bearer ", "");

  // Store the token
  await store.set("access_token", token);

  // Retrieve the user infos from the response
  let data: User;
  try {
    data = await response.json();

    if (!data) {
      throw new SyntaxError();
    }
  } catch (e) {
    console.error(e);
    throw new Error("Failed to login");
  }

  // Return the user infos
  return data;
}

export async function logout(token: string): Promise<void> {
  const appStore = useAppStore();

  let response: Response;
  try {
    response = await appStore.fetch("/api/auth", {
      method: "DELETE",
      headers: {
        Authorization: "Bearer " + token,
      },
    });
  } catch (e) {
    throw new Error("Failed to logout: " + e);
  }

  if (!response.ok) {
    console.error(response);
    throw new Error("Failed to logout: status code " + response.status);
  }
}

export async function refresh(token: string): Promise<User> {
  const appStore = useAppStore();

  let response: Response;
  try {
    response = await appStore.fetch("/api/auth", {
      method: "GET",
      headers: {
        Authorization: "Bearer " + token,
      },
    });
  } catch (e) {
    throw new Error("Failed to refresh: " + e);
  }

  if (response.status == 401) {
    throw new Error("Invalid token");
  }

  if (!response.ok) {
    console.error(response);
    throw new Error("Failed to refresh: status code " + response.status);
  }

  let data: User;
  try {
    data = await response.json();

    if (!data) {
      throw new SyntaxError();
    }
  } catch (e) {
    console.error(e);
    throw new Error("Failed to refresh");
  }

  return data;
}

export function getStoredToken(): Promise<string | null> {
  return store.get<string>("access_token");
}
