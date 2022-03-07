export function average(x: number[]): number {
  return x.reduce((p, c) => p + c, 0) / x.length
}
