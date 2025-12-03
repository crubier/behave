
export enum StatusEnum {
  STATUS_UNSPECIFIED = 0,
  STATUS_IDLE = 1,
  STATUS_REQUESTED = 2,
  STATUS_STARTED = 3,
  STATUS_SUSPENDED = 4,
  STATUS_SUCCEEDED = 5,
  STATUS_CANCELLED = 6,
  STATUS_REJECTED = 7,
}

export type Status = {
  status: StatusEnum
}