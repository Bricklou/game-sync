export class HttpError extends Error {
  public readonly response: Response;

  constructor(response: Response) {
    super(response.statusText);
    this.response = response;
  }
}

export class HttpValidationError<K extends string> extends Error {
  public readonly fields: Record<K, string[]>;

  public constructor(fields: Record<string, string[]>) {
    super("Validation error");
    this.fields = fields;
  }

  public static async fromResponse<T extends string>(
    response: Response,
  ): Promise<HttpValidationError<T>> {
    const data = (await response.json()) as {
      message: string;
      fields: Record<T, string[]>;
    };
    return new HttpValidationError(data.fields);
  }

  public static assert<T extends string>(
    error: unknown,
  ): error is HttpValidationError<T> {
    return error instanceof HttpValidationError;
  }
}
