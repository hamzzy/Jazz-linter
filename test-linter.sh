#!/bin/bash

# Create a temporary JavaScript file
cat << EOF > test.js
let x = 5;
let y = 10;
 console.log(x);
EOF

echo "Testing linter with default output format:"
./target/release/Jazz-linter test.js

echo -e "\nTesting linter with JSON output format:"
./target/release/Jazz-linter test.js --format json

# Clean up
rm test.js