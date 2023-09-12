import { invoke } from "@tauri-apps/api";

export class Color {
  public constructor(
    public r: number,
    public g: number,
    public b: number,
  ) {}

  public static fromHex(hex: string): Color {
    return new Color(
      parseInt(hex.substring(1, 3), 16),
      parseInt(hex.substring(3, 5), 16),
      parseInt(hex.substring(5, 7), 16),
    );
  }

  public toHex(): string {
    return `#${this.r.toString(16)}${this.g.toString(16)}${this.b.toString(
      16,
    )}`;
  }
}

export function getLuminance(color: Color): number {
  return 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
}

export async function getImageDominantColor(src: string): Promise<Color> {
  const result = await invoke<Color>("get_image_dominant", { src });

  console.log("result", result);

  return result;
}
