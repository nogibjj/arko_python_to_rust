# Performance Comparison of Rust and Python Data Processing Scripts

This report documents the performance comparison between the Rust and Python implementations for processing `AAPL.csv` data. Each script calculates the average closing price of Apple's stock, with execution time and resource usage recorded for comparison.

## Environment

- **CPU**: [AMD Ryzen 7]
- **RAM**: [16 GB]
- **OS**: [Ubuntu 20.04]
- **Rust Version**: [Run `rustc --version` to get version]
- **Python Version**: [Run `python --version` to get version]
- **Libraries**:
  - **Rust**: `polars`
  - **Python**: `pandas`

## Results

### Execution Time

| Metric               | Rust (ms)         | Python (ms)       |
|----------------------|-------------------|-------------------|
| Execution Time       |`274.328665`       |`2040`             |
| Average Close Price  |`87.25658896034298`|`87.25658896034298`|

### Memory Usage (Optional)

If you’ve measured memory, add a table here:

| Metric               | Rust (MB)         | Python (MB)       |
|----------------------|-------------------|-------------------|
| Peak Memory Usage    | `30.8`            | `5.52`            |

## Observations

- **Speed**: ["Rust was approximately 10x faster than Python."]
- **Memory Usage**: ["Rust consumed 6x more memory than Python."]
- **Development Experience**: ["Writing python code was faster and a more approchable process because of the easier to understand syntax."]


## Conclusion

This comparison highlights the trade-offs between using Rust and Python for data processing tasks. Rust’s high performance, as demonstrated by its 10x faster execution time, makes it a powerful choice for processing large datasets or performing high-throughput tasks where efficiency is paramount. Rust's strong memory management and zero-cost abstractions contribute to this speed, making it well-suited for applications where performance is critical and predictable resource usage is required.

On the other hand, Python, while slower, offers simplicity and ease of development, allowing for rapid prototyping and iterative development. Python’s extensive ecosystem of libraries, like Pandas, and its readable syntax make it ideal for data science workflows, exploratory analysis, and applications where development speed and flexibility are prioritized over raw performance.

### Preferred Use Cases
- **Rust**: Recommended for high-performance, resource-constrained applications, real-time data processing, and systems where execution speed is critical.
- **Python**: Suitable for exploratory data analysis, machine learning prototyping, and situations where ease of use and quick development cycles are more important than performance.

In summary, Rust and Python each bring unique strengths to data processing. Rust is optimal for production-grade, performance-critical applications, while Python remains a valuable tool for flexible, research-oriented, and iterative data science tasks.
