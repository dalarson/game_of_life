import { useCallback, useMemo, useState } from "react";
import { Square } from "./Square";
import { convert1DTo2D, convert2DTo1D } from "./boardUtils";
import type { LivingCells } from "./types";

export const Board = () => {
  const [age, setAge] = useState<number>(0);
  const [boardSize, setBoardSize] = useState<number>();
  const [livingCells, setLivingCells] = useState<LivingCells>([]);
  const [intervalId, setIntervalId] = useState<number | undefined>();

  const handleReset = () => {
    setAge(0);
    setLivingCells([]);
  };

  const go = () => {
    if (!intervalId) {
      setIntervalId(
        setInterval(() => {
          handleEvolve();
        }, 100),
      );
    }
  };

  const stop = () => {
    if (intervalId) {
      clearInterval(intervalId);
      setIntervalId(undefined);
    }
  };

  const handleRandomize = () => {
    const numberOfCells = Math.random() * (boardSize! * boardSize!);
    const newLivingCells: LivingCells = [];
    for (let i = 0; i < numberOfCells; i++) {
      const row = Math.floor(Math.random() * boardSize!);
      const col = Math.floor(Math.random() * boardSize!);
      newLivingCells.push([row, col]);
    }
    setLivingCells(newLivingCells);
  };

  // TODO: check for game completion
  const handleEvolve = () => {
    setLivingCells((oldLivingCells) => {
      const newLivingCells: LivingCells = [];
      for (let i = 0; i < boardSize! * boardSize!; i++) {
        const coords = convert1DTo2D(i, boardSize!);
        if (evolveCell(coords[0], coords[1], oldLivingCells))
          newLivingCells.push(coords);
      }
      return newLivingCells;
    });
    setAge((oldAge) => oldAge + 1);
  };

  // get the number of living neighbors that cell (x, y) has
  const getNeighbors = (
    x: number,
    y: number,
    livingCells: LivingCells,
  ): number => {
    return livingCells.filter((cell) => {
      return (
        // x coordinate is within 1 unit and y coordinate is within 1 unit
        Math.abs(cell[0] - x) <= 1 &&
        Math.abs(cell[1] - y) <= 1 &&
        // and not the cell itself
        !(cell[0] === x && cell[1] === y)
      );
    }).length;
  };

  // return whether the cell at (x, y) will be alive or dead in the next generation
  const evolveCell = (x: number, y: number, livingCells: LivingCells) => {
    const isCurrentlyAlive = livingCells.some(
      (cell) => cell[0] === x && cell[1] === y,
    );
    const livingNeighbors = getNeighbors(x, y, livingCells);

    if (isCurrentlyAlive) {
      // Any live cell with two or three live neighbors survives.
      if (livingNeighbors === 2 || livingNeighbors === 3) {
        return true;
      } else {
        // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
        return false;
      }
    } else {
      // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
      if (livingNeighbors === 3) {
        return true;
      }
      return false;
    }
  };

  const handleClick = useCallback(
    (index: number) => {
      const coordinates = convert1DTo2D(index, boardSize!);
      const cellIndex = livingCells.findIndex(
        (cell) => cell[0] === coordinates[0] && cell[1] === coordinates[1],
      );
      if (cellIndex > -1) {
        // Cell is currently alive, so we remove it
        const newLivingCells = [...livingCells];
        newLivingCells.splice(cellIndex, 1);
        setLivingCells(newLivingCells);
      } else {
        // Cell is currently dead, so we add it
        setLivingCells([...livingCells, [coordinates[0], coordinates[1]]]);
      }
    },
    [livingCells, boardSize],
  );

  /**
   * Function to determine if a cell at coordinate x is alive or dead
   * @param index
   * @returns
   */
  const isAlive = useCallback(
    (index: number): boolean => {
      const coordinates = convert1DTo2D(index, boardSize!);
      return livingCells.some(
        (cell) => cell[0] === coordinates[0] && cell[1] === coordinates[1],
      );
    },
    [livingCells, boardSize],
  );

  const grid = useMemo(() => {
    if (!boardSize) return null;
    const grid = [];
    for (let i = 0; i < boardSize; i++) {
      const row = [];
      for (let j = 0; j < boardSize; j++) {
        const index = convert2DTo1D(i, j, boardSize);
        row.push(
          <Square
            key={index}
            isAlive={isAlive(index)}
            onClick={() => handleClick(index)}
          ></Square>,
        );
      }
      grid.push(
        <div key={i} className="board-row">
          {row}
        </div>,
      );
    }
    return grid;
  }, [boardSize, handleClick, isAlive]);

  return (
    <div>
      <input
        type="number"
        placeholder="Enter board size"
        onKeyDown={(event) => {
          if (event.key === "Enter") {
            setBoardSize(Number(event.currentTarget.value));
          }
        }}
      />
      {boardSize && (
        <div>
          <button onClick={handleEvolve}>Evolve</button>
          <button onClick={handleReset}>Reset</button>
          <button onClick={handleRandomize}>Randomize</button>
          <button onClick={go}>Go!</button>
          <button onClick={stop}>Stop!!</button>
          <div>{age}</div>
          {grid}
        </div>
      )}
    </div>
  );
};
