export interface IAppStatus {
  version: string;
  name: string;
  status: "UP" | "DOWN";
  configured: boolean;
}
