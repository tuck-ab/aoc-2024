DAY=$1

# Validate day as a number
if ! [[ $DAY =~ ^[0-9]+ ]]
then
    echo "Day '$DAY' not a number (^[0-9]+)"
    exit 0
fi

# Validate day 1 <= DAY <= 25
if [ $DAY -gt 25 -o $DAY -lt 1 ]
then
    echo "Day '$DAY' not [1..25]"
    exit 0
fi

# Day mod file
RUST_DAY_FILE="src/day_${DAY}.rs"

# Check if the file already exists
if [ -f $RUST_DAY_FILE ]
then
    echo "ERROR: File '$RUST_DAY_FILE' already exists"
    exit 1
fi

# Create the new file
cat > $RUST_DAY_FILE <<- EOM
use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day${DAY};

impl Solution for Day${DAY} {
    fn part_1() {
        let data = load_input(${DAY});
        println!("Day ${DAY}, part 1")
    }

    fn part_2() {
        let data = load_input(${DAY});
        println!("Day ${DAY}, part 2")
    }
}
EOM

# Work out the greatest rust module less than target day
target_mod=$DAY
while [ -z $LINE ]
do
    let "target_mod = target_mod - 1"
    LINE=$(grep -n "mod day_${target_mod};" src/main.rs | cut -d: -f1)
done

# Add the mod
sed -i "s/mod day_${target_mod};/mod day_${target_mod};\nmod day_${DAY};/" src/main.rs

# Add the entrypoing in main
function generate_run_line {
    echo "        $1 => run::<day_$1::Day$1>(args.part),"
}
sed -i "s/$(generate_run_line $target_mod)/$(generate_run_line $target_mod)\n$(generate_run_line $DAY)/" src/main.rs

# Add the input file
source .env
curl \
    --cookie "_ga=${_ga};_gid=${_gid};session=$session"\
    "https://adventofcode.com/2024/day/${DAY}/input" \
    -o "inputs/day${DAY}.txt"

