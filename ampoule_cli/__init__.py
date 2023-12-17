from command_pb2 import Command, Response, Opcode
from serial import Serial
import time

def pong(transport):
    command = Command()
    command.opcode = Opcode.PING

    b = command.SerializeToString()
    size = int.to_bytes(len(b), length=2, byteorder="big")
    payload = size + b
    print(payload)

    transport.write(payload)
    b = transport.read(4)
    print("".join("{:02x}".format(x) for x in b))
    response = Response.FromString(b[2:])
    print(response)
    assert response.opcode == Opcode.PONG


def main():
    with Serial("/dev/ttyACM1", baudrate=115200) as transport:
        while True:
            pong(transport)
            time.sleep(0.001)


if __name__ == "__main__":
    main()
