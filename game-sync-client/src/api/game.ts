import { useAppStore } from "@/store/modules/app";
import { Game, GameInput } from "@/types/game";
import { HttpError, HttpValidationError } from "@/types/http_errors";
import { PaginationQuery, PaginationResponse } from "@/types/pagination";

export async function createGame(game: GameInput): Promise<void> {
  const appStore = useAppStore();

  let response: Response;
  try {
    response = await appStore.fetch("/api/games", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(game),
    });
  } catch (e) {
    throw new Error("Failed to create game: " + e);
  }

  if (response.status == 422) {
    throw await HttpValidationError.fromResponse(response);
  }

  if (!response.ok) {
    console.error(response);
    throw new HttpError(response);
  }
}

export async function getGames(
  input?: PaginationQuery,
): Promise<PaginationResponse<Game>> {
  const appStore = useAppStore();

  let response: Response;
  try {
    const params = new URLSearchParams();
    if (input?.page) params.set("page", input.page.toString());
    if (input?.perPage) params.set("per_page", input.perPage.toString());

    response = await appStore.fetch("/api/games?" + params.toString());
  } catch (e) {
    throw new Error("Failed to get games: " + e);
  }

  if (!response.ok) {
    console.error(response);
    throw new HttpError(response);
  }

  return await response.json();
}
