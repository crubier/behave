import type { ConnectRouter } from "@connectrpc/connect";
import { BehaviorService } from "@behave/types/behave/behavior/v1/behavior_pb";

export default (router: ConnectRouter) =>
  router.service(BehaviorService, {
    async sayHello(req) {
      console.log('sayHello', req);
      return {
        message: `Hello, ${req.name}!`
      };
    }
  });