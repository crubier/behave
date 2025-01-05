import { PrintArgument, PrintResult } from "@/gen/behaviors/print/v1/types_pb"

export function PrintArgumentView(props: { argument: PrintArgument }) {
  return <div>{props.argument.text}</div>
}

export function PrintResultView(props: { result: PrintResult }) {
  return <div>Ok</div>
}
