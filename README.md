# Proton

Crypto data is ubiquitous, "free", and public... yet it is largely unusable if trying to reference at scale. <br>
Proton sets out to create highly specialized compression algorithms for common datasets on a "column by column" approach. <br>
Proton will evenetually morph to a Rust crate, but for now is in alpha/early development.

## Why Proton?
- Crypto data is cumbersome, hard to decipher, and memory intensive.
- Historical data can be highly relevant, but not schematically available in many use cases.
- Datasets are often highly repetitive, following formats that could benefit from specialized compression.

## Project Overview/Introduction
- Proton early development is focused on the followign datasets: transfers, logs, and token metadata.
- Building on open source data extraction tools Proton aims to provide highly specialized compression for commonly used crypto datasets (transfers, logs, metadata, etc.). At present, Proton is focused on buiding through Parquet file types.

## Current Status
- Alpha/early development stage
- Testing compression ratios against standard solutions
- Exploring optimal algorithms for different data columns
- Planning migration to a Rust crate for production use