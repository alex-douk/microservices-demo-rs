#!/usr/bin/sh


FRONTEND_ADDR=$1 #"localhost:8000"
MAX_RETRIES=12
RETRY_INTERVAL=10
for i in $(seq 1 $MAX_RETRIES); do
  echo "Attempt $i: Pinging frontend: ${FRONTEND_ADDR}..."
  STATUSCODE=$(wget --server-response http://${FRONTEND_ADDR} 2>&1 | awk '/^  HTTP/{print $2}')
  if [ $STATUSCODE -eq 200 ]; then
      echo "Frontend is reachable."
      exit 0
  fi
  echo "Error: Could not reach frontend - Status code: ${STATUSCODE}"
  sleep $RETRY_INTERVAL
done
echo "Failed to reach frontend after $MAX_RETRIES attempts."
exit 1
