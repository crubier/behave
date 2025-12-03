import * as React from "react";
import { PrintArgument, PrintResult } from "./types"

export function PrintArgumentView(props: { argument: PrintArgument }) {
  return <div>{props.argument.text}</div>
}

export function PrintResultView(props: { result: PrintResult }) {
  return <div>Ok</div>
}
