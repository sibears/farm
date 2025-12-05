import logging
import socket
import random

READ_TIMEOUT = 5
BUFSIZE = 4096

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

submitted_flags = set()


def recv_line(sock: socket.socket) -> str:
    data = b""
    while True:
        try:
            chunk = sock.recv(1)
            if not chunk:
                break
            data += chunk
            if chunk == b"\n":
                break
        except socket.timeout:
            break
    return data.decode("utf-8", errors="ignore").strip()


def handle_flag(flag: str) -> str:
    if not flag:
        return "[ERR] Invalid format"
    
    if flag in submitted_flags:
        return "[ERR] Already submitted"
    
    choice = random.choices(
        ["OK", "ERR_INVALID_FORMAT", "ERR_INVALID_FLAG", "ERR_EXPIRED", "ERR_OWN_FLAG", "ERR_NOP_TEAM"],
        weights=[0.4, 0.1, 0.15, 0.1, 0.15, 0.1],
        k=1
    )[0]
    
    if choice == "OK":
        submitted_flags.add(flag)
        return "[OK]"
    elif choice == "ERR_INVALID_FORMAT":
        return "[ERR] Invalid format"
    elif choice == "ERR_INVALID_FLAG":
        return "[ERR] Invalid flag"
    elif choice == "ERR_EXPIRED":
        return "[ERR] Expired"
    elif choice == "ERR_OWN_FLAG":
        return "[ERR] This is your own flag"
    elif choice == "ERR_NOP_TEAM":
        return "[ERR] Can't submit flag from NOP team"
    
    return "[ERR] Invalid flag"


with socket.create_server(("0.0.0.0", 31337)) as sock:
    sock.settimeout(None)
    
    while True:
        try:
            conn, client_address = sock.accept()
            conn.settimeout(READ_TIMEOUT)
            logger.info(f"New connection from {client_address}")
            
            while True:
                try:
                    flag = recv_line(conn)
                    
                    if not flag:
                        logger.info(f"Connection closed by {client_address}")
                        conn.close()
                        break
                    
                    logger.info(f"Received flag from {client_address}: {flag}")
                    response = handle_flag(flag)
                    logger.info(f"Sending response: {response}")
                    
                    conn.sendall(response.encode("utf-8") + b"\n")
                    
                except socket.timeout:
                    logger.warning(f"Timeout waiting for flag from {client_address}")
                    conn.close()
                    break
                except Exception as e:
                    logger.error(f"Error handling flag from {client_address}: {e}")
                    conn.close()
                    break
                    
        except KeyboardInterrupt:
            logger.info("Shutting down...")
            break
        except Exception as e:
            logger.error(f"Server error: {e}")
            continue
