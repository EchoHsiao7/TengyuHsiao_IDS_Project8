import polars as pl
import time
import psutil


def read_data(file):
    df = pl.read_csv(file)
    return df


def generate_summary_statistics(dataframe, col_name):
    # Mean
    mean_value = dataframe[col_name].mean()
    # Median
    median_value = dataframe[col_name].median()
    # Standard deviation
    std_value = dataframe[col_name].std()

    print(f"Summary Statistics for {col_name}:")
    print(f"Mean: {mean_value}")
    print(f"Median: {median_value}")
    print(f"Standard Deviation: {std_value}")

    return mean_value, median_value, std_value


if __name__ == "__main__":
    process = psutil.Process()
    cpu_start = process.cpu_percent(interval=None)
    start_time = time.time()
    input_file = "Steam_2024_bestRevenue_1500.csv"
    df_data = read_data(input_file)
    colname = "price"  # Replace with the actual column name
    mean, median, std = generate_summary_statistics(df_data, colname)
    end_time = time.time()
    cpu_end = process.cpu_percent(interval=None)
    print("CPU Usage (%):", cpu_end)
    print("Execution time:", end_time - start_time)
