import { useState } from "react";
import { useDataStore } from "./store/githubStore";

function App() {
  const { data, status, getData } = useDataStore();
  const [value, setValue] = useState("");

  const handleClick = () => {
    if (value) {
      getData(value);
      return;
    }

    alert("Please enter a search query.");
  };

  return (
    <>
      <h1>Fetch with Zustand</h1>
      <input
        type="text"
        value={value}
        onChange={(e) => setValue(e.target.value)}
      />
      <button onClick={handleClick}>Search</button>
      {status === "IDLE" && <p>No data.</p>}
      {status === "LOADING" && <p>Loading...</p>}
      {status === "SUCCESS" && (
        <ul>
          {data.map((item) => (
            <li key={item.full_name}>
              <a href={item.html_url} target="_blank" rel="noreferrer noopener">
                {item.html_url}
              </a>
            </li>
          ))}
        </ul>
      )}
    </>
  );
}

export default App;
