import { store } from "./store";
import { fetch } from "@tauri-apps/plugin-http";

export async function getConfiguredServer(): Promise<string | null> {
  return await store.get<string | null>("server_url");
}

export async function setConfiguredServer(url: string | null): Promise<void> {
  await store.set("server_url", url);
  await store.save();
}

interface ServerInfos {
  name: string;
  version: string;
  configured: boolean;
  status: "UP" | "DOWN";
}

export async function validateServerUrl(value: string | null): Promise<void> {
  // Check if the url is empty
  if (!value || value.length === 0) {
    throw new Error("Server url cannot be empty");
  }

  // Check if the url is valid
  let url: URL;
  try {
    url = new URL(value);
    url.pathname = "/";
  } catch (e) {
    throw new Error("Invalid server url: " + e);
  }

  // Check if the url is reachable
  let response: Response;
  console.debug("Check server url", url.toString());
  try {
    response = await fetch(url, {
      method: "GET",
      connectTimeout: 1000,
    });
  } catch (e) {
    throw new Error("Failed to reach server: " + e);
  }

  if (!response.ok) {
    throw new Error("Failed to reach server: status code " + response.status);
  }

  // Check if the url is a valid server
  console.debug("Get server infos from response");
  console.log(response);
  let data: ServerInfos;
  try {
    data = await response.json();

    if (!data) {
      throw new SyntaxError();
    }
  } catch (e) {
    if (e instanceof Error && e.name === "SyntaxError") {
      throw new Error("Failed to get server infos: received data are invalid");
    }
    console.error(e);
    throw new Error("Failed to get server infos: " + e);
  }
  console.debug("Check server infos", data);

  if (data?.name !== "game-sync-server") {
    throw new Error("Invalid server");
  }

  if (data?.configured !== true) {
    throw new Error(
      "Server not configured, please visit the server url to configure it first",
    );
  }

  if (data?.status !== "UP") {
    throw new Error("Server is in a failure state, please try again later");
  }
}
