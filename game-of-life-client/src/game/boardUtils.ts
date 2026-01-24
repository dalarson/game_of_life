export const convert1DTo2D = (
  index: number,
  boardSize: number,
): [number, number] => {
  const row = Math.floor(index / boardSize);
  const col = index % boardSize;
  return [row, col];
};

export const convert2DTo1D = (
  row: number,
  col: number,
  boardSize: number,
): number => {
  return row * boardSize + col;
};
