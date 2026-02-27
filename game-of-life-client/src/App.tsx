import "./App.css";
import { Board } from "./game/Board";

function App() {
  return (
    <div className={"game-of-life"}>
      <h1>Conway's Game of Life</h1>
      <Board />
    </div>
  );
}

export default App;
