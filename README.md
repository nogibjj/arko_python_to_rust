[![Rust CI/CD](https://github.com/nogibjj/arko_python_to_rust/actions/workflows/CICD-rust.yml/badge.svg)](https://github.com/nogibjj/arko_python_to_rust/actions/workflows/CICD-rust.yml)
[![Python CI/CD](https://github.com/nogibjj/arko_python_to_rust/actions/workflows/CICD-python.yml/badge.svg)](https://github.com/nogibjj/arko_python_to_rust/actions/workflows/CICD-python.yml)
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

## Setup


```
git clone https://github.com/nogibjj/arko_python_to_rust.git
cd arko_python_to_rust
make rust_all
make all
```
Note:
for visualizing rust memory utilization: (the massif.out.<pid> file gets generated)
```
sudo apt-get update
sudo apt-get install valgrind
cargo build --release
valgrind --tool=massif target/release/aapl_data_processor
massif-visualizer massif.out.<pid>
```
## Results

### Execution Time

| Metric               | Rust (ms)         | Python (ms)       |
|----------------------|-------------------|-------------------|
| Execution Time       |`274.328665`       |`2040`             |
| Average Close Price  |`87.25658896034298`|`87.25658896034298`|

![image](https://github.com/user-attachments/assets/50077fff-8182-4f27-8561-81ccfee74d7e)

![image](https://github.com/user-attachments/assets/198756b7-9af0-4f03-9f85-c91207a7a7f6)


### Memory Usage (Optional)

If you’ve measured memory, add a table here:

| Metric               | Rust (MB)         | Python (MB)       |
|----------------------|-------------------|-------------------|
| Peak Memory Usage    | `30.8`            | `5.52`            |

![image](https://github.com/user-attachments/assets/d83e64f5-c082-49f6-8dce-e05485dd3913)

![image](https://github.com/user-attachments/assets/198756b7-9af0-4f03-9f85-c91207a7a7f6)

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
