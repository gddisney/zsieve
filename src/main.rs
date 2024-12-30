use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;
use rayon::prelude::*;
use num_complex::Complex;

#[derive(Debug)]
struct Resonance {
    prime: u64,
    tangent: f64,
    cosine: f64,
    sine: f64,
    angle: f64,
    gamma: f64,
    target_zero: Option<f64>,
    tangent_difference: Option<f64>,
}

/// Reads all zeta zeros into a sorted vector.
fn load_zeta_zeros(zeta_dir: &str) -> io::Result<Vec<f64>> {
    let mut zeta_zeros = Vec::new();
    let zeta_files: Vec<_> = fs::read_dir(zeta_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .collect();

    for zeta_file in zeta_files {
        if let Some(zeta_path) = zeta_file.to_str() {
            let file = File::open(zeta_path)?;
            for line in BufReader::new(file).lines() {
                if let Ok(zeta) = line?.parse::<f64>() {
                    zeta_zeros.push(zeta);
                }
            }
        }
    }

    zeta_zeros.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(zeta_zeros)
}

/// Saves results in bulk to minimize file I/O and validate data.
fn save_resonances_to_file(output_file: &str, resonances: &[Resonance]) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(output_file)?;
    for resonance in resonances {
        if let Some(zeta_zero) = resonance.target_zero {
            writeln!(
                file,
                "MATCH: Prime: {}, Tangent: {:.6}, Cosine: {:.6}, Sine: {:.6}, Angle: {:.6}, Gamma: {:.6}, Zeta Zero: {:.6}, Difference: {:.6}",
                resonance.prime,
                resonance.tangent,
                resonance.cosine,
                resonance.sine,
                resonance.angle,
                resonance.gamma,
                zeta_zero,
                resonance.tangent_difference.unwrap_or(0.0)
            )?;
        } else {
            writeln!(
                file,
                "NEW ZETA ZERO: Prime: {}, Tangent: {:.6}, Cosine: {:.6}, Sine: {:.6}, Angle: {:.6}, Gamma: {:.6}",
                resonance.prime,
                resonance.tangent,
                resonance.cosine,
                resonance.sine,
                resonance.angle,
                resonance.gamma
            )?;
        }
    }
    Ok(())
}

/// Approximate the Riemann zeta function for a given complex number.
fn compute_zeta(s: Complex<f64>, iterations: usize) -> Complex<f64> {
    (1..=iterations)
        .map(|n| Complex::new(n as f64, 0.0).powc(-s))
        .sum()
}

/// Validate if a gamma corresponds to a new zeta zero.
fn validate_zeta_zero(gamma: f64, threshold: f64, iterations: usize) -> bool {
    let s = Complex::new(0.5, gamma);
    let zeta_value = compute_zeta(s, iterations);
    zeta_value.norm() < threshold
}

/// Checks if a value matches any zeta zero using binary search.
fn find_closest_zeta_zero(tangent: f64, zeta_zeros: &[f64], tolerance: f64) -> Option<(f64, f64)> {
    let pos = zeta_zeros.binary_search_by(|z| z.partial_cmp(&tangent).unwrap()).unwrap_or_else(|e| e);
    if pos < zeta_zeros.len() {
        let diff = (tangent - zeta_zeros[pos]).abs();
        if diff <= tolerance {
            return Some((zeta_zeros[pos], diff));
        }
    }
    None
}

/// Processes primes in parallel to calculate resonances.
fn process_primes_parallel(
    prime_files: Vec<std::path::PathBuf>,
    zeta_zeros: Vec<f64>,
    tolerance: f64,
    output_file: &str,
) -> io::Result<()> {
    prime_files.par_iter().for_each(|prime_file| {
        let mut resonances = Vec::new();
        if let Some(prime_path) = prime_file.to_str() {
            let file = File::open(prime_path).unwrap();
            for line in BufReader::new(file).lines() {
                if let Ok(prime) = line.unwrap().parse::<u64>() {
                    let tangent = (prime as f64).tan();
                    if !tangent.is_finite() {
                        continue;
                    }

                    let cosine = tangent.cos();
                    let angle = tangent.atan();
                    let sine = angle.sin();
                    let gamma = angle / (2.0 * std::f64::consts::PI);

                    if let Some((zeta_zero, diff)) = find_closest_zeta_zero(tangent, &zeta_zeros, tolerance) {
                        println!(
                            "MATCH FOUND: Prime: {}, Tangent: {:.6}, Cosine: {:.6}, Sine: {:.6}, Angle: {:.6}, Gamma: {:.6}, Zeta Zero: {:.6}, Difference: {:.6}",
                            prime, tangent, cosine, sine, angle, gamma, zeta_zero, diff
                        );
                        resonances.push(Resonance {
                            prime,
                            tangent,
                            cosine,
                            sine,
                            angle,
                            gamma,
                            target_zero: Some(zeta_zero),
                            tangent_difference: Some(diff),
                        });
                    } else if cosine.abs() < 0.05 && sine.abs() > 0.99 && angle.abs() >= 1.5 {
                        if validate_zeta_zero(gamma, 1e-6, 10_000) {
                            println!(
                                "NEW ZETA ZERO CONFIRMED: Prime: {}, Tangent: {:.6}, Cosine: {:.6}, Sine: {:.6}, Angle: {:.6}, Gamma: {:.6}",
                                prime, tangent, cosine, sine, angle, gamma
                            );
                            resonances.push(Resonance {
                                prime,
                                tangent,
                                cosine,
                                sine,
                                angle,
                                gamma,
                                target_zero: None,
                                tangent_difference: None,
                            });
                        } 
                    }
                }
            }
        }
        save_resonances_to_file(output_file, &resonances).unwrap();
    });

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: find-zeta-primes <tolerance> <zeta_dir> <prime_dir> <output_file>");
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Not enough arguments")));
    }

    let tolerance = f64::from_str(&args[1])?;
    let zeta_dir = &args[2];
    let prime_dir = &args[3];
    let output_file = &args[4];

    println!("Loading zeta zeros...");
    let zeta_zeros = load_zeta_zeros(zeta_dir)?;
    println!("Loaded {} zeta zeros.", zeta_zeros.len());

    println!("Scanning prime directory...");
    let prime_files: Vec<_> = fs::read_dir(prime_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .collect();

    println!("Processing primes in parallel...");
    process_primes_parallel(prime_files, zeta_zeros, tolerance, output_file)?;

    println!("Program completed successfully.");
    Ok(())
}

