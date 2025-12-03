import { PrintRunner } from "./print/v1/run"
import { SequenceRunner } from "./sequence/v1/run"
import { TextRunner } from "./text/v1/run"

export const Runners = {
  print: PrintRunner,
  sequence: SequenceRunner,
  text: TextRunner
}