[![Python CI](https://github.com/EchoHsiao7/TengyuHsiao_IDS_Project8/actions/workflows/cicd.yml/badge.svg)](https://github.com/EchoHsiao7/TengyuHsiao_IDS_Project8/actions/workflows/cicd.yml)

[![CI](https://github.com/EchoHsiao7/TengyuHsiao_IDS_Project8/actions/workflows/CI.yml/badge.svg)](https://github.com/EchoHsiao7/TengyuHsiao_IDS_Project8/actions/workflows/CI.yml)


# Tengyu Hsiao Proejct 8


## This project: 

1. Take an existing Python script for data processing

2. Rewrite it in Rust

3. Highlight improvements in speed and resource usage

## Improvement in speed and resource

In python, I use psutil and time to measure the CPU resource and execution time of the script, here is the result:
CPU Usage (%): 3.5
Execution time: 0.0033

In Rust, I use analysis and time to measure the CPU resource and execution time of the script, here is the result:

CPU Usage (%): 6.5
Execution time (wall-clock time): 0.001278 seconds

It can be noticed that Rust has a faster execution time, but it would take a bit more CPU resource for this data processing project.
