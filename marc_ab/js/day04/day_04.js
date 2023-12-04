/*
===========================
--- Day 4: Scratchcards ---
===========================

by Marc Abillama
https://github.com/mabillama
*/

// --- Part One ---
const fs = require("fs");

let sample;
try {
  sample = fs.readFileSync(__dirname + "/day_04_input", "utf8");
} catch (err) {
  console.error(err);
}

function decipherScratchcards(scratchcardData) {
  return scratchcardData.split(/\r?\n/).map((cardLine) => {
    const numericPart = cardLine.substring(cardLine.indexOf(":") + 1).trim();
    const [winningNumbers, chanceNumbers] = numericPart
      .split("|")
      .map((side) => side.trim().split(/\s+/).map(Number));
    return [winningNumbers, chanceNumbers];
  });
}

function calculateScratchcardLuck(winningNumbers, chanceNumbers) {
  const luckyMatches = chanceNumbers.filter((num) =>
    winningNumbers.includes(num)
  );
  return luckyMatches.length > 0 ? Math.pow(2, luckyMatches.length - 1) : 0;
}

function countScratchcardPoints(cardsString) {
  const funDeck = decipherScratchcards(cardsString);
  const pointsPile = funDeck.map(([winningNumbers, playerNumbers]) =>
    calculateScratchcardLuck(winningNumbers, playerNumbers)
  );
  const totalTreasure = pointsPile.reduce((acc, points) => acc + points, 0);
  return totalTreasure;
}

console.time("Part 1 Execution Time");
const totalPoints = countScratchcardPoints(sample);
console.log(
  `PART 1 - The total points in the elf's colorful card pile is: ${totalPoints}`
);
console.timeEnd("Part 1 Execution Time");

// --- Part Two ---
function calculateCardMatches(winningNumbers, chanceNumbers) {
  return chanceNumbers.filter((num) => winningNumbers.includes(num)).length;
}

function processScratchcards(allCards, startIndex = 0) {
  const [winningNumbers, chanceNumbers] = allCards[startIndex];
  const matches = calculateCardMatches(winningNumbers, chanceNumbers);

  const newCardsIndices = Array.from(
    { length: matches },
    (_, i) => startIndex + 1 + i
  ).filter((index) => index < allCards.length);

  const newCardsCount = newCardsIndices.reduce(
    (count, index) => count + processScratchcards(allCards, index),
    0
  );

  return 1 + newCardsCount;
}

function calculateTotalScratchcards(cardsString) {
  const allCards = decipherScratchcards(cardsString);
  return allCards.reduce(
    (acc, _, index) => acc + processScratchcards(allCards, index),
    0
  );
}
console.time("Part 2 Execution Time");
const totalScratchcards = calculateTotalScratchcards(sample);
console.log(
  `PART 2 - The total number of scratchcards after processing is: ${totalScratchcards}`
);
console.timeEnd("Part 2 Execution Time");
