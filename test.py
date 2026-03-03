import socket
import random
import json
import uuid
from datetime import datetime
import threading

HOST = "127.0.0.1"
PORT = 7878
THREADS = 100
LOGS_PER_THREAD = 10000

levels = [
    "Trace", "Debug", "Info", "Notice",
    "Warning", "Error", "Critical"
]

def generate_log():
    return {
        "id": str(uuid.uuid4()),
        "timestamp": datetime.utcnow().replace(microsecond=0).isoformat() + "Z",
        "level": random.choice(levels),
        "service": "load-test",
        "environment": "dev",
        "message": random.choice([
            "User logged in",
            "Fetching data from API",
            "Connection timeout",
            "Invalid credentials",
            "Database updated successfully",
        ]),
        "correlation_id": None,
        "request_id": None,
        "span_id": None
    }

def worker(thread_id):
    print(f"[Thread {thread_id}] Connecting...")
    sock = socket.create_connection((HOST, PORT))
    print(f"[Thread {thread_id}] Connected!")

    for _ in range(LOGS_PER_THREAD):
        log_json = json.dumps(generate_log())
        sock.sendall((log_json + "\n").encode("utf-8"))
        
    sock.shutdown(socket.SHUT_WR)
    sock.close()

    print(f"[Thread {thread_id}] Finished sending.")

def main():
    threads = []

    for i in range(THREADS):
        t = threading.Thread(target=worker, args=(i,))
        t.start()
        threads.append(t)

    for t in threads:
        t.join()

    print("All threads finished.")

if __name__ == "__main__":
    main()