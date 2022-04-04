# Benford Analyzer v0.1
---

Benford Analyzer is a development experiment. A pet project to help me learn Rust by making a small sample project that will analyze data as per Benford's Law.

[More info about Benford's Law here](https://en.wikipedia.org/wiki/Benford's_law)

What this program will do is read a csv file with series of data and analyze the values as per Benford's Law in terms of the begining digit of the values.

The structure of the input file should be like:

| Value 1   | Value 2 | ... |
|-----------|---------|-----|
| 17823     | 23200   | ....|
| ...       | ...     | ... |

Each value column will be used a separate series to be analyzed. The minimum content of the file should contain at least one column of data to be read

The end result will have to show a display of the validity of the data in some form of a chart that will indicate compliance (or not) with Benford's Law.

## Program Goals
---
An initial goal is to make this program as "handmade" as possible. Since this is a learning project, in the intial iterations of the code will not make use of existing libs, as for example would be to use a parser for the CSV files and the command line arguments

The first milestone to achieve is reading an input file (csv), evaluate the data and calculate the results of the values as per Benford's Law. Once that is complete a visual depiction should be displayed. At first an ASCII chart of some sort will suffice.

Once this is complete, the next milestone to achieve is to implement some sort of actual Graphical Depiction for the results.

As a next milestone, we might attempt to implement an interface that will be more flexible in regards to input sources (e.g. network streams, other file types etc)
