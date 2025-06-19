const letters = "abcdefghijklmnopqrstuvwxyz";

function randomPositiveIntegerSmallerThan(upperLimit: number) {
  return Math.floor(Math.random() * upperLimit)
}

export function generateRandomStringOf10LowerLetters(): string {
  let randomString = ""
  for (let i = 0; i < 10; i++) {
    randomString += letters[randomPositiveIntegerSmallerThan(letters.length)]
  }
  return randomString
}