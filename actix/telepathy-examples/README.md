# Telepathy Examples

This repository is a collection of examples built with the [Actix-Telepathy](https://github.com/wenig/actix-telepathy) library.

## Examples

- [paper-example](./paper-example/): A simple example to demonstrate the easy transition from an [Actix](https://github.com/actix/actix) to an [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application.
- [PowerPingPong](./PowerPingPong/) A distributed [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application that sends byte messages back and forth but every time increases the message size by the power of 2.
- [Primes Calculation](./Primes%20Calculation/) A distributed [Actix-Telepathy](https://github.com/wenig/actix-telepathy) application that calculates all primes up to a given number by distributing the search space to a pool of workers.
- [Alternative Implementations](./alternative-implementations/) Holds a zip file with implementations of [PowerPingPong](./PowerPingPong/) and [Primes Calculation](./Primes%20Calculation/) implemented in [Akka](https://github.com/akka/akka) and [Orleans](https://github.com/dotnet/orleans).
