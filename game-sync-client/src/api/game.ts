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

export async function getGames({
  pagination = undefined,
  sortOrder = "asc",
  search,
}: {
  pagination?: PaginationQuery;
  sortOrder: "asc" | "desc";
  search?: string;
}): Promise<PaginationResponse<Game>> {
  const appStore = useAppStore();

  let response: Response;
  try {
    const params = new URLSearchParams();
    if (pagination?.page) params.set("page", pagination.page.toString());
    if (pagination?.perPage)
      params.set("per_page", pagination.perPage.toString());

    params.set("sort_order", sortOrder);

    if (search) params.set("search", search);

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

export async function getGame(id: number): Promise<Game> {
  const appStore = useAppStore();

  let response: Response;

  try {
    response = await appStore.fetch("/api/games/" + id);
  } catch (e) {
    throw new Error("Failed to get game: " + e);
  }

  if (!response.ok) {
    console.error(response);
    throw new HttpError(response);
  }

  return await response.json();
}
