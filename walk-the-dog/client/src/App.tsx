import init, { draw_sierpinski_triangle, greet } from "@/lib/wasm";
import { useEffect } from "react";
import "./App.css";

function App() {
  useEffect(() => {
    init();
  }, []);
  return (
    <>
      <h1>Vite + React + WASM</h1>
      <div
        className="card"
        style={{
          display: "flex",
          justifyContent: "center",
          gap: "1rem",
        }}
      >
        <div>
          <button onClick={() => greet()}>greet</button>
        </div>
        <div>
          <button onClick={() => draw_sierpinski_triangle()}>draw</button>
        </div>
      </div>
      <canvas id="canvas" tabIndex={0} height={600} width={600}>
        Your browser doesn't support canvas.
      </canvas>
    </>
  );
}

export default App;
