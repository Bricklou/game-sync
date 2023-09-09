export interface PaginationQuery {
  page?: number;
  perPage?: number;
}

export interface PaginationMeta {
  current_page: number;
  total_items: number;
  total_pages: number;
  per_page: number;
}

export interface PaginationResponse<T> {
  data: T[];
  meta: PaginationMeta;
}
