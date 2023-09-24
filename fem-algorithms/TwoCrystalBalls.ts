export default function two_crystal_balls(breaks: boolean[]): number {
  // given two identical glass balls, find the best way to figure out
  // what floor of a building they will break from. you can drop them
  // over and over until they break, but you cannot acquire more balls.
  // the building is an array of booleans, true meaning they broke in
  // previous testing, and false meaning they didn't.
  let lo = 0;
  const hi = breaks.length;
  const step = Math.sqrt(hi);

  // figure out when the first ball breaks, in steps of sqrt(n)
  do {
    if (breaks[lo]) {
      console.log("first ball breaks at", lo);
      break;
    } else {
      lo += step;
    }
  } while (lo <= hi);

  // backtrack to the last known-good floor and linear search from there to find the actual floor
  for (let i = lo - step; i < lo; i++) {
    if (breaks[i]) {
      return i;
    }
  }

  // return something nonsensical if it doesn't break
  return -1;
}
