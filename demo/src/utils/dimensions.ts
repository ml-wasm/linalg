export enum Dimension {
  None,
  Zero,
  One,
  Two,
}
export function dimension(x: any): Dimension {
  if (x === undefined || x === null) return Dimension.None;
  if (typeof x === 'number') return Dimension.Zero;
  if (typeof x === 'object' && typeof x[0] === 'number') return Dimension.One;
  return Dimension.Two;
}
