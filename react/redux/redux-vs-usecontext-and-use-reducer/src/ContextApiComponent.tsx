import { createContext, useContext } from "react";

const MyContext = createContext("Hello, Context!");

function ParentComponent() {
  const data = "My context data";
  return (
    <MyContext.Provider value={data}>
      <ChildComponent />
    </MyContext.Provider>
  );
}

function ChildComponent() {
  const value = useContext(MyContext);
  return (
    <>
      <h1>createContext + useContext</h1>
      <p>{value}</p>
    </>
  );
}

export { ParentComponent as ContextApiComponent };
