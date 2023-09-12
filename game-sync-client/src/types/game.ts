export interface GameInput {
  name: string;
  description?: string;
}

export enum GameBannerType {
  Color = "color",
  Image = "image",
}

export interface GameBanner {
  id: number;
  game_id: number;
  banner_type: GameBannerType;
  value: string;
}

export interface Game {
  id: number;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;

  banner?: GameBanner;
}
