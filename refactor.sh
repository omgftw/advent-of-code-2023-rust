#!/usr/bin/env bash

cur_day=$1

if [ -z "$cur_day" ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

mkdir src/day"$cur_day"
mkdir src/day"$cur_day"/data
mv src/day"$cur_day".rs src/day"$cur_day"/mod.rs
mv src/data/"$cur_day".txt src/day"$cur_day"/data/main.txt
mv src/test_data/"$cur_day".txt src/day"$cur_day"/data/test_1.txt

find='src\/data\/'"$cur_day.txt"
replace='src\/day'"$cur_day"'\/data\/main.txt'
sed -i.bak "s/$find/$replace/" src/day"$cur_day"/mod.rs

find='src\/test_data\/'"$cur_day"'.txt'
replace='src\/day'"$cur_day"'\/data\/test_1.txt'
sed -i.bak "s/$find/$replace/" src/tests.rs

rm src/day"$cur_day"/mod.rs.bak
rm src/tests.rs.bak
