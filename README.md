# 83

my first longer code attempt at generating a number for the counting channel

## What it do

using my prng that i wrote for anura (adapted to use u16 instead of u64 so this doesn't take 89310853 years), it tests seeds until it finds one that the first value is the number we are searching for, in this case 83, and then outputs it