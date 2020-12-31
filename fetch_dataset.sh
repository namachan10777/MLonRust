#!/bin/sh

cd examples

mkdir -p wine_quality
cd wine_quality
wget https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-red.csv
wget https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-white.csv
wget https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality.names
