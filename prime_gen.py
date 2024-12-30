import math
import os

def segmented_sieve(limit, output_dir, segment_size=10**8):
    """
    Generates all prime numbers up to a given limit using the segmented sieve algorithm.
    Writes primes to files in batches for large-scale generation.

    Args:
        limit (int): The upper limit for prime generation.
        output_dir (str): Directory to store output files containing primes.
        segment_size (int): Size of each segment for processing (default: 10^8).

    Returns:
        None
    """
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Step 1: Find small primes up to sqrt(limit)
    sqrt_limit = int(math.sqrt(limit)) + 1
    is_prime = [True] * sqrt_limit
    is_prime[0], is_prime[1] = False, False  # 0 and 1 are not primes

    for num in range(2, int(math.sqrt(sqrt_limit)) + 1):
        if is_prime[num]:
            for multiple in range(num * num, sqrt_limit, num):
                is_prime[multiple] = False

    small_primes = [i for i, prime in enumerate(is_prime) if prime]
    print(f"Small primes calculated up to {sqrt_limit}.")

    # Step 2: Use segmented sieve for the entire range up to `limit`
    start = 0
    file_count = 1
    while start < limit:
        end = min(start + segment_size, limit)
        segment = [True] * (end - start)
        
        # Mark non-primes in the segment
        for prime in small_primes:
            # Find the start index in the current segment
            first_multiple = max(prime * prime, start + (prime - start % prime) % prime)
            for j in range(first_multiple, end, prime):
                segment[j - start] = False

        # Extract primes from the segment
        primes = [i + start for i, prime in enumerate(segment) if prime]

        # Write primes to a file
        output_file = os.path.join(output_dir, f"primes_{file_count}.txt")
        with open(output_file, "w") as f:
            f.write("\n".join(map(str, primes)))
        print(f"Primes from {start} to {end} written to {output_file}.")
        file_count += 1

        start = end

if __name__ == "__main__":
    upper_limit = 10**12  # 1 trillion
    output_directory = "primes_up_to_1t"
    segmented_sieve(upper_limit, output_directory, segment_size=10**8)

import math
import os

def segmented_sieve(limit, output_dir, segment_size=10**8):
    """
    Generates all prime numbers up to a given limit using the segmented sieve algorithm.
    Writes primes to files in batches for large-scale generation.

    Args:
        limit (int): The upper limit for prime generation.
        output_dir (str): Directory to store output files containing primes.
        segment_size (int): Size of each segment for processing (default: 10^8).

    Returns:
        None
    """
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Step 1: Find small primes up to sqrt(limit)
    sqrt_limit = int(math.sqrt(limit)) + 1
    is_prime = [True] * sqrt_limit
    is_prime[0], is_prime[1] = False, False  # 0 and 1 are not primes

    for num in range(2, int(math.sqrt(sqrt_limit)) + 1):
        if is_prime[num]:
            for multiple in range(num * num, sqrt_limit, num):
                is_prime[multiple] = False

    small_primes = [i for i, prime in enumerate(is_prime) if prime]
    print(f"Small primes calculated up to {sqrt_limit}.")

    # Step 2: Use segmented sieve for the entire range up to `limit`
    start = 0
    file_count = 1
    while start < limit:
        end = min(start + segment_size, limit)
        segment = [True] * (end - start)
        
        # Mark non-primes in the segment
        for prime in small_primes:
            # Find the start index in the current segment
            first_multiple = max(prime * prime, start + (prime - start % prime) % prime)
            for j in range(first_multiple, end, prime):
                segment[j - start] = False

        # Extract primes from the segment
        primes = [i + start for i, prime in enumerate(segment) if prime]

        # Write primes to a file
        output_file = os.path.join(output_dir, f"primes_{file_count}.txt")
        with open(output_file, "w") as f:
            f.write("\n".join(map(str, primes)))
        print(f"Primes from {start} to {end} written to {output_file}.")
        file_count += 1

        start = end

if __name__ == "__main__":
    from sys import argv
    upper_limit = 10**12  # 1 trillion
    output_directory = argv[1] 
    segmented_sieve(upper_limit, output_directory, segment_size=10**10)

