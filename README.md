# EJ Kmer Dispatcher

EJ Kmer Dispatcher is a demonstration tool that showcases how to build custom applications using the [EJ Dispatcher SDK](https://crates.io/crates/ej-dispatcher-sdk).
This tool programmatically submits jobs to an EJ Dispatcher, retrieves results, and performs automated validation of the [k-mer algorithm benchmark](https://github.com/embj-org/kmer)
across different algorithm versions.

It was built as part of the EJ Guide series that can be found [here](https://embj-org.github.io/ej/).

## What It Does

This dispatcher client demonstrates advanced EJ Dispatcher SDK features including:

- Programmatically dispatches build and run jobs to EJ Dispatcher
- Fetches and parses job results from the dispatcher
- Verifies that different k-mer implementations produce consistent results

## Installation

### From Git

```bash
cargo install --git https://github.com/embj-org/ejkmer-dispatcher
```

### From Source

```bash
git clone https://github.com/embj-org/ejkmer-dispatcher.git
cd ejkmer-dispatcher
cargo install --path .
```

## Usage

### Dispatch Run Jobs

Submit a complete build-and-run job to the dispatcher:

```bash
ejkmer-dispatcher dispatch-run \
    --socket /path/to/ejd.sock \
    --seconds 60 \
    --commit-hash eb7c6cbe6249aff4df82455bbadf4898b0167d09 \
    --remote-url https://github.com/embj-org/kmer
```

## Example Output

```bash
ejkmer-dispatcher dispatch-run --socket ~/ejd.sock --seconds 60 --commit-hash eb7c6cb --remote-url https://github.com/embj-org/kmer

=======================================
Run finished successfully with 3 log entries:
=======================================
[Build logs and run logs displayed...]

=======================================
Run finished successfully with 3 result entries:
=======================================
k-mer-original: Results: ABC: 2, BCD: 1, CDA: 1, DAB: 1
k-mer:          Results: ABC: 2, BCD: 1, CDA: 1, DAB: 1  
k-mer-omp:      Results: ABC: 2, BCD: 1, CDA: 1, DAB: 1

Results OK!
```

## Integration with EJ Guides

This project serves as the primary example in [EJ Guide 04 - Dispatcher SDK](https://embj-org.github.io/ej/04-DispatcherSDK.html), 
demonstrating how to build specialized tools around the EJ infrastructure, and a simple example showcasing parsing and validating the test results.

It's a simple example that doesn't use the full capabilities of the `Dispatcher SDK` like retrieving previous results.

## Related Projects

- [K-mer Benchmark](https://github.com/embj-org/kmer) - The application being tested
- [EJ Kmer Builder](https://github.com/embj-org/ejkmer-builder) - The builder that deploys the application
- [EJ Framework](https://github.com/embj-org/ej) - The testing framework infrastructure
- [EJ Dispatcher SDK](https://crates.io/crates/ej-dispatcher-sdk) - The SDK this project demonstrates

## Contributing

Contributions are welcome! This project serves as a reference implementation for the EJ Dispatcher SDK, so improvements that demonstrate additional SDK capabilities or
analysis techniques are particularly valuable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For questions about this dispatcher client or the EJ framework:

- Check the [EJ Documentation](https://embj-org.github.io/ej/)
- Visit the [EJ GitHub Repository](https://github.com/embj-org/ej)
- Review the [Dispatcher SDK Documentation](https://crates.io/crates/ej-dispatcher-sdk)
