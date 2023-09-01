import { invoke } from "@tauri-apps/api";

export async function get_configured_server(): Promise<string | null> {
  const { url } = await invoke<{ url: string | null }>("get_configured_server");
  console.log("url: ", url);
  return url;
}
