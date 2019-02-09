

class AppError extends Error {

  constructor(public readonly code: string, public readonly message: string = 'no message', public readonly original?: Error | any) {
    super(message);

    this.name = this.constructor.name;
  }

}
