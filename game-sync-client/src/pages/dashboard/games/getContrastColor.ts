import { GameBanner, GameBannerType } from "@/types/game";
import { Color, getImageDominantColor, getLuminance } from "@/utils/colors";

/**
* Get contrast color from a hex color or an image
* @param banner_data The banner data with type (image or color) and value (hex color or image url)
* @returns The contrast color
*/
export async function getContrastColor(banner_data: GameBanner): string {
let color: Color;

// If the banner is a color, we can get the contrast color
if (banner_data.banner_type === GameBannerType.Color) {
const hex = banner_data.value;
color = Color.fromHex(hex);
} else {
// If the banner is an image, get the average color of the image
color = await getImageDominantColor(banner_data.value);
}

// Counting the perceptive luminance - human eye favors green color...
// https://stackoverflow.com/a/3943023/1232793
const luminance = getLuminance(color);

return luminance > 186 ? "text-gray-800" : "text-white";
}
