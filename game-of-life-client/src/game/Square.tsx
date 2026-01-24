import "./square.css";

interface SquareProps {
  isAlive: boolean;
  onClick: () => void;
}

export const Square = ({ isAlive, onClick }: SquareProps) => {
  return (
    <button
      className={`square ${isAlive ? "alive" : "dead"}`}
      onClick={onClick}
    ></button>
  );
};
