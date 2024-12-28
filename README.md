# zsieve

zsieve is a high-performance Rust program designed to analyze the relationship between prime numbers and the nontrivial zeros of the Riemann zeta function. It utilizes trigonometric and Fourier analysis to uncover resonance patterns, periodicities, and relationships that offer insights into the distribution of primes and their connection to zeta zeros.

## Features

- **Prime-Zeta Zero Mapping**:
  Identifies and maps primes to their corresponding nontrivial zeta zeros by leveraging trigonometric functions like tangent, cosine, and sine.

- **Precision Matching**:
  Operates at precision levels as fine as `1e-6` or smaller, ensuring highly accurate calculations.

- **Fourier Transform Analysis**:
  Applies Fourier analysis to uncover periodicities in the distribution of primes relative to zeta zeros.

- **Performance**:
  Optimized for high-performance computation, capable of processing large datasets of primes and zeta zeros efficiently.
  
## Cacluated primes from the first 200 zeta zeros at 1e-6

| Prime Number     | Tangent    | Cosine     | Sine       | Angle (rad) | Zeta Zero   | Difference  |
|------------------|------------|------------|------------|-------------|-------------|-------------|
| 100385452031     | 79.337376  | 0.012603   | 0.999921   | 1.558193    | 79.337375   | 0.000001    |
| 10140075821      | 14.134725  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 105721362283     | 107.168611 | -0.009331  | 0.999956   | 1.561466    | 107.168611  | 0.000000    |
| 10596600407      | 25.010857  | -0.039951  | 0.999202   | 1.530835    | 25.010857   | 0.000000    |
| 106076906983     | 25.010857  | 0.039951   | 0.999202   | 1.530835    | 25.010857   | 0.000000    |
| 108144821137     | 21.022038  | -0.047515  | 0.998871   | 1.523263    | 21.022039   | 0.000001    |
| 109650399059     | 14.134725  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 11209042717      | 14.134724  | -0.070571  | 0.997507   | 1.500166    | 14.134725   | 0.000001    |
| 112910892173     | 150.925256 | 0.006626   | 0.999978   | 1.564171    | 150.925257  | 0.000001    |
| 113576904811     | 94.651343  | -0.010565  | 0.999944   | 1.560232    | 94.651344   | 0.000001    |
| 14556597431      | 21.022040  | -0.047515  | 0.998871   | 1.523263    | 21.022039   | 0.000001    |
| 15239059379      | 14.134725  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 16172079467      | 79.337376  | 0.012603   | 0.999921   | 1.558193    | 79.337375   | 0.000001    |
| 18390633061      | 60.831779  | 0.016437   | 0.999865   | 1.554359    | 60.831778   | 0.000001    |
| 21507989719      | 107.168612 | -0.009331  | 0.999956   | 1.561466    | 107.168611  | 0.000001    |
| 2903158471       | 14.134725  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 30726533969      | 60.831778  | 0.016437   | 0.999865   | 1.554359    | 60.831778   | 0.000000    |
| 30740410079      | 30.424875  | -0.032850  | 0.999460   | 1.537940    | 30.424876   | 0.000001    |
| 31655311823      | 37.586178  | -0.026596  | 0.999646   | 1.544197    | 37.586178   | 0.000000    |
| 3359683057       | 25.010858  | -0.039951  | 0.999202   | 1.530835    | 25.010857   | 0.000001    |
| 34424565793      | 84.735493  | -0.011801  | 0.999930   | 1.558995    | 84.735492   | 0.000001    |
| 34811877637      | 14.134724  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000001    |
| 36703960507      | 14.134725  | -0.070571  | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 36894484423      | 60.831777  | -0.016437  | 0.999865   | 1.554359    | 60.831778   | 0.000001    |
| 37364623129      | 40.918719  | -0.024431  | 0.999702   | 1.546362    | 40.918719   | 0.000000    |
| 45009844753      | 14.134725  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000000    |
| 45832960727      | 14.134726  | 0.070571   | 0.997507   | 1.500166    | 14.134725   | 0.000001    |
| 47106327649      | 30.424877  | 0.032850   | 0.999460   | 1.537940    | 30.424876   | 0.000001    |
| 47534283493      | 21.022038  | 0.047515   | 0.998871   | 1.523263    | 21.022039   | 0.000001    |
| 52633267051      | 21.022039  | 0.047515   | 0.998871   | 1.523263    | 21.022039   | 0.000000    |
| 54558867017      | 147.422765 | 0.006783   | 0.999977   | 1.564013    | 147.422765  | 0.000000    |
| 55247680703      | 32.935061  | 0.030349   | 0.999539   | 1.540443    | 32.935061   | 0.000000    |
| 58018812937      | 65.112544  | 0.015356   | 0.999882   | 1.555440    | 65.112544   | 0.000000    |
| 59442228557      | 30.424876  | 0.032850   | 0.999460   | 1.537940    | 30.424876   | 0.000000    |
| 60497319343      | 60.831779  | 0.016437   | 0.999865   | 1.554359    | 60.831778   | 0.000001    |
| 60967458049      | 40.918720  | 0.024431   | 0.999702   | 1.546362    | 40.918719   | 0.000001    |
| 61415631157      | 32.935061  | -0.030349  | 0.999539   | 1.540443    | 32.935061   | 0.000000    |
| 61426097197      | 37.586178  | -0.026596  | 0.999646   | 1.544197    | 37.586178   | 0.000000    |

## Installation

To get started with zsieve, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### Clone the Repository

```bash
git clone https://github.com/gddisney/zsieve.git
cd zsieve
```

### Build the Project

```bash
cargo build --release
```

### Run the Program

```bash
./target/release/zsieve <tolerance> <zeta_dir> <prime_dir> <output_file>
```

### Arguments
- `<tolerance>`: Precision level for prime-zeta zero resonance matching (e.g., `1e-6`).
- `<zeta_dir>`: Directory containing files of zeta zeros.
- `<prime_dir>`: Directory containing files of primes.
- `<output_file>`: Path to save the output results.

## Usage Example

```bash
./target/release/zsieve 1e-6 ./data/zeta ./data/primes ./output/results.txt
```

## Output Format

The output file contains matched primes and zeta zeros along with their associated trigonometric values:

```text
Prime: 10140075821, Tangent: 14.134725, Cosine: 0.070571, Sine: 0.997507, Angle: 1.500166, Zeta Zero: 14.134725, Difference: 0.000000
Prime: 105721362283, Tangent: 107.168611, Cosine: -0.009331, Sine: 0.999956, Angle: 1.561466, Zeta Zero: 107.168611, Difference: 0.000000
```

## Mathematical Framework

zsieve leverages mathematical principles to map primes to zeta zeros:

```math
\zeta(s) = \sum_{n=1}^\infty \frac{1}{n^s}, \quad \Re(s) > 1
```

The nontrivial zeros of the zeta function are those for which:

```math
\zeta(\rho) = 0, \quad \rho = \frac{1}{2} + it
```

The program computes trigonometric values for primes and compares them to zeta zeros using:

```math
\text{Tangent Difference} = |\tan(\text{Prime}) - \text{Zeta Zero}|
```

## Results

zsieve identifies resonance points where the tangent of a prime number closely matches a zeta zero, with differences as small as `1e-6`. For example:

```text
Prime: 100385452031, Tangent: 79.337376, Zeta Zero: 79.337375, Difference: 0.000001
```

## Contributions

Contributions are welcome! If you have suggestions for improvements, new features, or bug fixes, feel free to create an issue or submit a pull request.

### How to Contribute

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Commit your changes with clear and concise messages.
4. Submit a pull request.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](./LICENSE) file for details.

## Acknowledgments

zsieve is inspired by the pursuit of understanding the Riemann Hypothesis and its implications for prime distribution. Special thanks to the open-source community for their support and contributions.

## Contact

For questions, suggestions, or feedback, please open an issue or reach out to the repository maintainer.
