#!/bin/bash

echo "Stopping Node.js Agent Farm..."

# Kill any process listening on the agent ports
fuser -k 4000/tcp 4001/tcp 4002/tcp 2>/dev/null || true

echo "Agents stopped successfully."
