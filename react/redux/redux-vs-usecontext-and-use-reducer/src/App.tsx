// https://www.cythilya.tw/2023/05/25/implement-redux-by-react-context-api-and-useReducer/

import { Greeting } from "./Greeting";
import { ReducerContextProvider } from "./ReducerContextProvider";

function App() {
  return (
    <ReducerContextProvider>
      <Greeting />
    </ReducerContextProvider>
  );
}

export default App;
