# Fourier.rs
Implementing a Fast Fourier Transform (FFT) and a Discrete Fourier Transform (DFT) in the Rust programming language and benchmarking the results.

## Fourier Transforms
Fourier transforms specify how much of each frequency is present in a signal, transforming the signal from the time domain to the frequency domain. The naive method is to use a Discrete Fourier Transform (DFT), which takes O(n^2) operations, where n is the input size of the signal. 

This is computationally expensive, as opposed to the Fast Fourier Transform (FFT), which uses pre-computed values corresponding to powers of 2 to complete Fourier Transforms in O(n log n) operations, a significant step up from the DFT.

## The Rust Programming Language
Memory safety and speed have always been seen as contradicting aspects of any programming language. In order to write extremely fast, high performant code, one must have deep knowledge of machine architecture and manual memory management for languages like C/C++ and assembly. In order to solve this, most languages (Python, JS, optionally Java) use a garbage collector to clean up memory leaks at the expense of speed.

Rust presents a new paradigm where memory management does not rely on the user while maintaining C-like speed. This new memory paradigm is called Ownership & Borrowing. In simple terms, data is owned by the current function scope and cannot be modified by functions outside of this scope. Data can be borrowed (eg. referenced) by other functions outside of the scope, but cannot be modified. All of this is enforced at compile time, ensuring memory safety without reducing performance.

## Code Overview
The plan was to implement both a FFT and a DFT using no imports. In order to create the FFT, I used the Cooley-Tukey Algorithm as a reference. First, I created a Complex number type as a struct, with basic complex number operations. Next, I created functions to reverse the bits of the input so that it is easier for the FFT algorithm to process. The twiddle function represents {e^ -2(pi*i)}. Putting all of this into two 'for loops', the functions were complete and ready for testing.

## Methodology
After implementing the algorithms, I created a benchmark file containing functions to measure the speed of the algorithms for exponentially increasing input sizes. I used a minimum  and maximum power of 2 for the input sizes (eg. first input size was 2^3 and final input size was 2^16). For every input size, I performed 5 trials measuring the speed. This data was outputted from a helper function that prints the results to the terminal in CSV format. From there, I plotted the results using Excel.

## Results
As expected, the FFT implementation absolutely crushed the DFT implementation. The difference was so large, that I had to take the natural log of the runtime for each implementation for every input size.

<img width="600" height="371" alt="image" src="https://github.com/user-attachments/assets/243686aa-a951-4cb4-b94b-d346d6b6b481" />


## References
GitHub:
Cooley-Tukey Algorithm: 
