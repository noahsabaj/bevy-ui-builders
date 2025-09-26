#!/bin/bash
echo "Starting text input demo - click on inputs to test cursor visibility"
echo "Expected behavior:"
echo "  1. Click on input -> cursor appears (white line)"
echo "  2. Type text -> cursor moves with text"
echo "  3. Click another input -> cursor moves to new input"
echo "  4. Click outside -> cursor disappears"
echo ""
echo "Starting in 3 seconds..."
sleep 3
cargo run --example text_input_demo --features text_input