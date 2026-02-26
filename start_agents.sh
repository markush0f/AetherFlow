#!/bin/bash

# Ensure we kill all background processes when the script exits (e.g. via Ctrl+C)
trap 'echo "Shutting down the agents..."; kill 0' EXIT

echo "Starting Node.js Agent Farm..."
(cd agents/translator && npm run start)

# Wait for all background processes to finish
wait
