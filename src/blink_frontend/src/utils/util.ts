export function convert<T>(input: [] | [T] | undefined): T | undefined {
  if (input === undefined) {
    return undefined;
  } else if (input.length === 0) {
    return undefined;
  } else {
    return input[0];
  }
}


export function getTime(timestamp: number): string {
  // Create a new JavaScript Date object based on the timestamp
  // multiplied by 1000 so that the argument is in milliseconds, not seconds
  const date = new Date(timestamp * 1000);
  const hours = date.getHours();
  const minutes = "0" + date.getMinutes();
  const formattedTime = hours + ':' + minutes.substr(-2);

  return formattedTime
}

export function trimStr(string: string): string {
  return string.length > 25 ? string.substring(0, 25 - 3) + "..." : string
}
