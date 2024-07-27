const GIGA_SECONDS = 1_000_000_000;

function main() {
    const date = new Date();
    const timeInMillSecondsFromEpoch = date.getTime();
    const gigaMillseconds = GIGA_SECONDS * 1000;
    return new Date(timeInMillSecondsFromEpoch + gigaMillseconds);
}

console.log('time from now to giga seconds more', main());