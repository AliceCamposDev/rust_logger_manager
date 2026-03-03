import socket
import random
from datetime import datetime
import threading

HOST = "127.0.0.1"
PORT = 7878
THREADS = 10
LOGS_PER_THREAD = 10000

levels = ["INFO", "DEBUG", "WARNING", "ERROR"]

def generate_log():
    level = random.choice(levels)
    message = random.choice([
        "User logged in",
        "Fetching data from API",
        "Connection timeout",
        "Invalid credentials",
        "Database updated successfully",
    ])
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return f"{timestamp} | {level} | {message}"

def worker(thread_id):
    print(f"[Thread {thread_id}] Connecting...")
    sock = socket.create_connection((HOST, PORT))
    print(f"[Thread {thread_id}] Connected!")

    for i in range(LOGS_PER_THREAD):
        log_line = generate_log()
        sock.sendall((log_line + "\n").encode("utf-8"))

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