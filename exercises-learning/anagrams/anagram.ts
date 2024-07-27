function getAnagramCandidates(word: string, candidates: string[]): string[] {
  const wordToMatch = word.toLowerCase();
  function filterCandidates(candidateWord: string): boolean {
    const lowerCasedCandidateWord = candidateWord.toLowerCase();
    if (wordToMatch === lowerCasedCandidateWord) {
      return false;
    }
    if (wordToMatch.length !== lowerCasedCandidateWord.length) {
      return false;
    }

    let isSameWordPresent = (function () {
      let newwordLength = [...new Set(lowerCasedCandidateWord)].length;
      return newwordLength !== wordToMatch.length;
    })();

    if (isSameWordPresent) {
      return false;
    }

    const lowerCasedCandidateWordSplitted = lowerCasedCandidateWord.split("");
    return lowerCasedCandidateWordSplitted.every((character) =>
      wordToMatch.includes(character)
    );
  }

  return candidates.filter(filterCandidates);
}
