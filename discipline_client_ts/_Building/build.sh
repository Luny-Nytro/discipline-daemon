#!/bin/bash

# Compile TypeScript into JavaScript
deno run --allow-all --unstable npm:typescript/tsc

# Resolve import aliases
deno run --allow-all ./Building/go.ts