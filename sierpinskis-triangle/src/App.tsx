import { useEffect, useState } from "react";
import init, { draw_triangle, greet } from "~/sierpinskis_triangle/pkg";
import "./App.css";

function App() {
  useEffect(() => {
    init();
    draw_triangle();
  }, []);
  const [count, setCount] = useState(0);
  return (
    <>
      <h1>Vite + React + WASM</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={() => greet()}>greet</button>
      </div>
      <canvas id="canvas" tabIndex={0} height={600} width={600}>
        Your browser doesn't support canvas.
      </canvas>
    </>
  );
}

export default App;
