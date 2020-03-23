function largestPrimeFactor(number) {
    var factors = factorizeByOddNumbers(number);

    return factors[factors.length - 1];
}

function factorizeByOddNumbers(number) {
    var factors = [];
    var temporaryNumber = number;

    while (temporaryNumber % 2 == 0 && temporaryNumber > 1) {
        factors.push(2);
        temporaryNumber /= 2;
    }

    for (var i = 3; i <= number && temporaryNumber > 1; i += 2) {
        if (temporaryNumber % i == 0) {
            factors.push(i);
            temporaryNumber /= i;
        }
    }

    return factors;
}

function factorizeByPrimes(number) {
    var primes = getAllPrimes(number);

    var temporaryNumber = number;
    var currentPrimeIndex = 0;
    var factors = [];
    while (temporaryNumber > 1 && currentPrimeIndex < primes.length) {
        var currentPrime = primes[currentPrimeIndex];

        if (temporaryNumber % currentPrime == 0) {
            factors.push(currentPrime);
            temporaryNumber /= currentPrime;
        } else {
            currentPrimeIndex += 1;
        }
    }

    return factors;
}

function isPrime(number) {
    var isPrime = true;

    var highestPossible = Math.sqrt(number);
    for (var i = 2; i <= highestPossible; i++) {
        if (number % i == 0) {
            isPrime = false;
            break;
        }
    }

    return isPrime;
}

function getAllPrimes(upperLimit) {
    var primes = [];
    for (var currentNumber = 2; currentNumber <= upperLimit; currentNumber++) {
        if (isPrime(currentNumber)) {
            primes.push(currentNumber);
        }
    }

    return primes;
}

largestPrimeFactor(13195);
