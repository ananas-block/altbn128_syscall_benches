<p align="center">
  <a href="https://solana.com">
    <img alt="Solana" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

[![Solana crate](https://img.shields.io/crates/v/solana-core.svg)](https://crates.io/crates/solana-core)
[![Solana documentation](https://docs.rs/solana-core/badge.svg)](https://docs.rs/solana-core)
[![Build status](https://badge.buildkite.com/8cc350de251d61483db98bdfc895b9ea0ac8ffa4a32ee850ed.svg?branch=master)](https://buildkite.com/solana-labs/solana/builds?branch=master)
[![codecov](https://codecov.io/gh/solana-labs/solana/branch/master/graph/badge.svg)](https://codecov.io/gh/solana-labs/solana)

# Syscall Benches

```
cd sdk
cargo +nightly bench alt_bn128
```

## Results

To be on the safe side let's choose 7ms as an upper bound for the execution time for one pairing.
Assuming 33ns per CU: 7ms * 1_000_000 / 33 (ns per CU) = 212121 Compute units per pairing



- pairing x benchmarks bench the execution of x pairings.
- pairing 1 rnd benchmarks bench the execution of 1 pairings with random input ten times.
- pairing 2 rnd benchmarks bench the execution of 2 pairings with random input ten times.


pairing 2               time:   [13.547 ms 13.646 ms 13.770 ms]
Found 12 outliers among 100 measurements (12.00%)
1 (1.00%) high mild
11 (11.00%) high severe

pairing 3               time:   [20.399 ms 20.553 ms 20.739 ms]
Found 14 outliers among 100 measurements (14.00%)
4 (4.00%) high mild
10 (10.00%) high severe

pairing 4               time:   [27.359 ms 27.628 ms 27.949 ms]
Found 11 outliers among 100 measurements (11.00%)
2 (2.00%) high mild
9 (9.00%) high severe

pairing 5               time:   [33.745 ms 33.862 ms 34.026 ms]
Found 17 outliers among 100 measurements (17.00%)
3 (3.00%) high mild
14 (14.00%) high severe

pairing 6               time:   [40.511 ms 40.582 ms 40.667 ms]
Found 15 outliers among 100 measurements (15.00%)
3 (3.00%) high mild
12 (12.00%) high severe

pairing 7               time:   [49.100 ms 50.109 ms 51.251 ms]
Found 12 outliers among 100 measurements (12.00%)
6 (6.00%) high mild
6 (6.00%) high severe

Benchmarking pairing 8: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, or reduce sample count to 90.
pairing 8               time:   [53.962 ms 54.053 ms 54.167 ms]
Found 13 outliers among 100 measurements (13.00%)
4 (4.00%) high mild
9 (9.00%) high severe

Benchmarking pairing 9: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.1s, or reduce sample count to 80.
pairing 9               time:   [61.057 ms 61.381 ms 61.760 ms]
Found 14 outliers among 100 measurements (14.00%)
4 (4.00%) high mild
10 (10.00%) high severe

pairing 1 rnd           time:   [6.8716 ms 6.9636 ms 7.0709 ms]
Found 12 outliers among 100 measurements (12.00%)
8 (8.00%) high mild
4 (4.00%) high severe

pairing 1 rnd #2        time:   [6.7214 ms 6.7588 ms 6.8031 ms]
Found 14 outliers among 100 measurements (14.00%)
2 (2.00%) high mild
12 (12.00%) high severe

pairing 1 rnd #3        time:   [6.7146 ms 6.7419 ms 6.7758 ms]
Found 14 outliers among 100 measurements (14.00%)
4 (4.00%) high mild
10 (10.00%) high severe

pairing 1 rnd #4        time:   [6.8448 ms 6.9190 ms 6.9999 ms]
Found 16 outliers among 100 measurements (16.00%)
11 (11.00%) high mild
5 (5.00%) high severe

pairing 1 rnd #5        time:   [6.7057 ms 6.7252 ms 6.7494 ms]
Found 21 outliers among 100 measurements (21.00%)
1 (1.00%) high mild
20 (20.00%) high severe

pairing 1 rnd #6        time:   [6.7152 ms 6.7503 ms 6.7954 ms]
Found 17 outliers among 100 measurements (17.00%)
2 (2.00%) high mild
15 (15.00%) high severe

pairing 1 rnd #7        time:   [6.8923 ms 7.0133 ms 7.1538 ms]
Found 19 outliers among 100 measurements (19.00%)
6 (6.00%) high mild
13 (13.00%) high severe

pairing 1 rnd #8        time:   [6.6804 ms 6.6971 ms 6.7274 ms]
Found 5 outliers among 100 measurements (5.00%)
2 (2.00%) high mild
3 (3.00%) high severe

pairing 1 rnd #9        time:   [6.6962 ms 6.7040 ms 6.7141 ms]
Found 8 outliers among 100 measurements (8.00%)
1 (1.00%) high mild
7 (7.00%) high severe

pairing 2 rnd           time:   [13.810 ms 14.001 ms 14.214 ms]
Found 7 outliers among 100 measurements (7.00%)
5 (5.00%) high mild
2 (2.00%) high severe

pairing 2 rnd #2        time:   [13.455 ms 13.458 ms 13.462 ms]
Found 3 outliers among 100 measurements (3.00%)
3 (3.00%) high mild

pairing 2 rnd #3        time:   [13.454 ms 13.458 ms 13.464 ms]
Found 5 outliers among 100 measurements (5.00%)
1 (1.00%) high mild
4 (4.00%) high severe

pairing 2 rnd #4        time:   [13.866 ms 14.146 ms 14.474 ms]
Found 19 outliers among 100 measurements (19.00%)
4 (4.00%) high mild
15 (15.00%) high severe

pairing 2 rnd #5        time:   [13.524 ms 13.621 ms 13.750 ms]
Found 15 outliers among 100 measurements (15.00%)
3 (3.00%) high mild
12 (12.00%) high severe

pairing 2 rnd #6        time:   [13.475 ms 13.524 ms 13.591 ms]
Found 17 outliers among 100 measurements (17.00%)
5 (5.00%) high mild
12 (12.00%) high severe

pairing 2 rnd #7        time:   [13.502 ms 13.559 ms 13.629 ms]
Found 20 outliers among 100 measurements (20.00%)
7 (7.00%) high mild
13 (13.00%) high severe

pairing 2 rnd #8        time:   [13.511 ms 13.574 ms 13.649 ms]
Found 19 outliers among 100 measurements (19.00%)
3 (3.00%) high mild
16 (16.00%) high severe

pairing 2 rnd #9        time:   [13.457 ms 13.484 ms 13.522 ms]
Found 13 outliers among 100 measurements (13.00%)
2 (2.00%) high mild
11 (11.00%) high severe

# Building

## **1. Install rustc, cargo and rustfmt.**

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup component add rustfmt
```

When building the master branch, please make sure you are using the latest stable rust version by running:

```bash
$ rustup update
```

When building a specific release branch, you should check the rust version in `ci/rust-version.sh` and if necessary, install that version by running:
```bash
$ rustup install VERSION
```
Note that if this is not the latest rust version on your machine, cargo commands may require an [override](https://rust-lang.github.io/rustup/overrides.html) in order to use the correct version.

On Linux systems you may need to install libssl-dev, pkg-config, zlib1g-dev, etc.  On Ubuntu:

```bash
$ sudo apt-get update
$ sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make
```

On Mac M1s, make sure you set up your terminal & homebrew [to use](https://5balloons.info/correct-way-to-install-and-use-homebrew-on-m1-macs/) Rosetta. You can install it with:

```bash
$ softwareupdate --install-rosetta
```

## **2. Download the source code.**

```bash
$ git clone https://github.com/solana-labs/solana.git
$ cd solana
```

## **3. Build.**

```bash
$ cargo build
```

# Testing

**Run the test suite:**

```bash
$ cargo test
```

### Starting a local testnet
Start your own testnet locally, instructions are in the [online docs](https://docs.solana.com/cluster/bench-tps).

### Accessing the remote development cluster
* `devnet` - stable public cluster for development accessible via
devnet.solana.com. Runs 24/7. Learn more about the [public clusters](https://docs.solana.com/clusters)

# Benchmarking

First install the nightly build of rustc. `cargo bench` requires use of the
unstable features only available in the nightly build.

```bash
$ rustup install nightly
```

Run the benchmarks:

```bash
$ cargo +nightly bench
```

# Release Process

The release process for this project is described [here](RELEASE.md).

# Code coverage

To generate code coverage statistics:

```bash
$ scripts/coverage.sh
$ open target/cov/lcov-local/index.html
```

Why coverage? While most see coverage as a code quality metric, we see it primarily as a developer
productivity metric. When a developer makes a change to the codebase, presumably it's a *solution* to
some problem.  Our unit-test suite is how we encode the set of *problems* the codebase solves. Running
the test suite should indicate that your change didn't *infringe* on anyone else's solutions. Adding a
test *protects* your solution from future changes. Say you don't understand why a line of code exists,
try deleting it and running the unit-tests. The nearest test failure should tell you what problem
was solved by that code. If no test fails, go ahead and submit a Pull Request that asks, "what
problem is solved by this code?" On the other hand, if a test does fail and you can think of a
better way to solve the same problem, a Pull Request with your solution would most certainly be
welcome! Likewise, if rewriting a test can better communicate what code it's protecting, please
send us that patch!

# Disclaimer

All claims, content, designs, algorithms, estimates, roadmaps,
specifications, and performance measurements described in this project
are done with the Solana Foundation's ("SF") good faith efforts. It is up to
the reader to check and validate their accuracy and truthfulness.
Furthermore, nothing in this project constitutes a solicitation for
investment.

Any content produced by SF or developer resources that SF provides, are
for educational and inspirational purposes only. SF does not encourage,
induce or sanction the deployment, integration or use of any such
applications (including the code comprising the Solana blockchain
protocol) in violation of applicable laws or regulations and hereby
prohibits any such deployment, integration or use. This includes use of
any such applications by the reader (a) in violation of export control
or sanctions laws of the United States or any other applicable
jurisdiction, (b) if the reader is located in or ordinarily resident in
a country or territory subject to comprehensive sanctions administered
by the U.S. Office of Foreign Assets Control (OFAC), or (c) if the
reader is or is working on behalf of a Specially Designated National
(SDN) or a person subject to similar blocking or denied party
prohibitions.

The reader should be aware that U.S. export control and sanctions laws
prohibit U.S. persons (and other persons that are subject to such laws)
from transacting with persons in certain countries and territories or
that are on the SDN list. As a project based primarily on open-source
software, it is possible that such sanctioned persons may nevertheless
bypass prohibitions, obtain the code comprising the Solana blockchain
protocol (or other project code or applications) and deploy, integrate,
or otherwise use it. Accordingly, there is a risk to individuals that
other persons using the Solana blockchain protocol may be sanctioned
persons and that transactions with such persons would be a violation of
U.S. export controls and sanctions law. This risk applies to
individuals, organizations, and other ecosystem participants that
deploy, integrate, or use the Solana blockchain protocol code directly
(e.g., as a node operator), and individuals that transact on the Solana
blockchain through light clients, third party interfaces, and/or wallet
software.
