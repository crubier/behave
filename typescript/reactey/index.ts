// console.log("OK")

// type StateTree = number[] | StateTree[];
// type ComponentPath = number[];
// type RenderState = { componentPath: ComponentPath, stateTree: StateTree, useStateIndex: number }

// const currentRenderStateSingleton: RenderState = { componentPath: [], stateTree: [], useStateIndex: 0 };

// const useState = (initializer) => {
//   if (currentRenderStateSingleton.componentPath.length <= 0) {
//     throw new Error("Not rendering anything!")
//   }
//   let currentState = currentRenderStateSingleton.stateTree;
//   for (const path in currentRenderStateSingleton.componentPath) {
//     if (currentState[path] != null) {
//       currentState = currentState[path];
//     }
//     else {
//       currentState[path] = []
//     }
//   }
//   let currentRenderStateSingleton.useStateIndex = 0;

// }

// const createBehaveComponent = <In, Out>(func: (props: In) => Out) => {
//   const state = {};
//   const result = {
//     state: state,
//     setState: (newState) => { result.state = newState },
//     render: func
//   };
// }


// const renderBehaveComponent = (component) => {
//   for (let i = 0; i < 100; i++) {
//     component.render
//   }
// }



// const printText = ({ text }: { text: string }) => {
//   // const [state, setState] = useState();
//   if (state)

//     return console.log("Text")
// }

// renderBehaveComponent(createBehaveComponent(printText))

enum Command {
  UNKNOWN,
  RUN,
  RESTART,
  STOP,
}

enum Status {
  UNKNOWN,
  PENDING,
  SUCCESS,
  FAILURE
}

const printText = ({ text }: { text: string }) => () => {
  console.log(text)
  return Status.SUCCESS
}

const fallback = ({ children }: { children: any[] }) => {
  for (const child in children) {
    if (child() != Status.SUCCESS) {
      return Status.FAILURE;
    }
  }
}

const myApp = () => {
  return fallback({
    children:
      [printText({ text: "Hello" })
      ]
  })
}

const renderBehaveComponent = (component) => {
  for (let i = 0; i < 100; i++) {
    component()
  }
}

renderBehaveComponent(myApp)

