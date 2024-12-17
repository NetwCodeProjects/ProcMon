import socket

HOST = '192.168.1.230'  # Replace with your internal IP
PORT = 9999             # Replace with your chosen port

with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as server:
    server.bind((HOST, PORT))
    print(f"Listening on {HOST}:{PORT}")
    while True:
        data, addr = server.recvfrom(1024)
        print(f"Received message: {data.decode()} from {addr}")
