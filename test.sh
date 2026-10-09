# First, declare variables for the function.
# Use command substitution in a language-agnostic way.

port="${PORT:-8080}"
pay_to="${PAY_TO:-0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045}"
network="${KITE_NETWORK:-mainnet}"
upstream="${UPSTREAM_URL:-https://api.open-meteo.com}"
price="${PRICE_USD:-0.001}"

echo "==> Testing x402 proxy (paying to $pay_to on $network)"
echo "==> Port: $port, Upstream: $upstream, Price: \$$price"

# 1. Health check (free, no payment required)
echo ""
echo "--- Test 1: Health check ---"
curl -s "http://localhost:$port/healthz" | python3 -m json.tool 2>/dev/null || \
  curl -s "http://localhost:$port/healthz"

# 2. Unpaid request to /v1/ should return 402
echo ""
echo "--- Test 2: Unpaid /v1/ request (expect 402) ---"
status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:$port/v1/forecast?latitude=52.52&longitude=13.41")
if [ "$status" = "402" ]; then
  echo "PASS: Got HTTP 402 as expected"
else
  echo "FAIL: Expected 402, got $status"
fi

# 3. Show the 402 response body
echo ""
echo "--- Test 3: 402 Response Body ---"
curl -s "http://localhost:$port/v1/forecast?latitude=52.52&longitude=13.41" | head -c 500

echo ""
echo "=== All tests complete ==="
