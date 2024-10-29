import pandas as pd
import time
from memory_profiler import memory_usage


def process_data(file_path):
    start_time = time.time()
    mem_usage_before = memory_usage(-1, interval=0.1, timeout=1)

    df = pd.read_csv(file_path)
    avg_close = df["Close(t)"].mean()

    mem_usage_after = memory_usage(-1, interval=0.1, timeout=1)
    end_time = time.time()

    execution_time = end_time - start_time
    memory_used = mem_usage_after[0] - mem_usage_before[0]

    print(f"Average closing price: {avg_close}")
    print(f"Execution time: {execution_time:.2f} seconds")
    print(f"Memory used: {memory_used:.2f} MB")


if __name__ == "__main__":
    process_data("data/AAPL.csv")
