// https://www.cythilya.tw/2023/05/25/implement-redux-by-react-context-api-and-useReducer/

import { GreetingWithRtk } from "./GreetingWithRtk";
import { Provider } from "react-redux";
import { store } from "./store";

function App() {
  return (
    <Provider store={store}>
      <GreetingWithRtk />
    </Provider>
  );
}

export default App;
