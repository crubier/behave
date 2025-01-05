import { createClient } from "@connectrpc/connect";
import { BehaviorService } from "@behave/types/behave/behavior/v1/behavior_pb";
import { createConnectTransport } from "@connectrpc/connect-node";

const transport = createConnectTransport({
  baseUrl: "http://localhost:8080",
  httpVersion: "2"
});

async function main() {
  const client = createClient(BehaviorService, transport);
  const res = await client.sayHello({ name: "Vincent" });
  console.log(res);
}
void main();