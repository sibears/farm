import socket
import time
import random

READ_TIMEOUT = 5
APPEND_TIMEOUT = 0.5
BUFSIZE = 4096

def recvall(sock):
    BUFF_SIZE = 4096 # 4 KiB
    data = b''
    while True:
        part = sock.recv(BUFF_SIZE)
        data += part
        if len(part) < BUFF_SIZE:
            # either 0 or end of data
            break
    return data


with socket.create_server(('0.0.0.0', 8778)) as sock:
    while True:
        connect, client_address = sock.accept()
        connect.send(b"Enter your flags, finished with newline (or empty line to exit)\n")
        while True:
            try:
                flag = recvall(connect).decode()
                if (flag == b'\n'):
                    connect.send(b"Goodbye!")
                    connect.close()
                    break
                flag = flag.strip()
                choice = random.randint(0, 3)
                if (choice == 0):
                    msg = f"[{flag}] Accepted. {random.randint(10, 100)} flag points"
                elif (choice == 1):
                    msg = f"[{flag}] Denied: you already submitted this flag"
                elif (choice == 2):
                    msg = f"[{flag}] Denied: no such flag"
                else:
                    msg = f"[{flag}] Denied: flag is your own"
                connect.send(msg.encode() + b'\n')
            except:
                connect.close()
                break
