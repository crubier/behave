import { Status } from "../../../commons/types"

export type SequenceArgument<AnyArgument> = {
  children: AnyArgument[]
}

export type SequenceStatus<AnyStatus extends Status> = {
  status: Status
}

export type SequenceResult = {}

export type SequenceState = {}

export type SequenceInput = {}

export type SequenceOutput = {}
