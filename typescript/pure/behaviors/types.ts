import { PrintArgument } from "./print/v1/types";
import { SequenceArgument } from "./sequence/v1/types";
import { TextArgument } from "./text/v1/types";

export type AnyArgument =
  | PrintArgument
  | SequenceArgument<any>
  | TextArgument;