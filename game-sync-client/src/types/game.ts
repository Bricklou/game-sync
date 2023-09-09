export interface GameInput {
  name: string;
  description?: string;
}

export interface Game {
  id: number;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}
