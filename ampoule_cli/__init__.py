from command_pb2 import Command, Response, Opcode
from serial import Serial
from itertools import chain


def main():
    with Serial('/dev/ttyACM0', baudrate=115200) as transport:
        command = Command()
        command.opcode = Opcode.PING

        b = command.SerializeToString()
        size = int.to_bytes(len(b), length=2, byteorder="big")
        payload = size + b
        print(payload)

        transport.write(payload)
        b = transport.read()
        response = Response.FromString(b)
        print(response)
        assert response.opcode == Opcode.PONG



if __name__ == "__main__":
    main()
