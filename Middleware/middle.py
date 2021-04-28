

import asyncio
import websockets
import socket
import json
from Encryption import EncryptionHandler 


class Middle:

    def __init__(self):
        timeout_placeholder ={}
        timeout_placeholder["status"] = "failed"
        self.timeout_response = json.dumps(timeout_placeholder)
        self.host = '127.0.0.1' #localhost
        self.port = 50222
        self.server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.Eh = EncryptionHandler()
        self.running = True
    

    async def check_status_and_continue(self,status,websocket,client):
        if status == "success":
            success_data = self.Eh.decrypted(await websocket.recv())
            client.send(success_data.encode("utf8"))
        else:
            client.send("issue".encode("utf8"))

    #send request to external server
    async def send_request(self ,data,client ):
         async with websockets.connect('ws://localhost:8765') as websocket:
             server_greeting = await websocket.recv()

             if self.Eh.decrypted(server_greeting) == "WHAT":
                 encrypted_request  = self.Eh.encrypted(data)
                 await websocket.send(encrypted_request)#encrypted request in json form
                 status = self.Eh.decrypted(await websocket.recv())#accepted password?
                 self.check_status_and_continue(status,websocket,client)

             else:
                 print(f"Server Responded with :{self.Eh.decrypted(server_greeting)}")
            
    #listens for request from dir guardian client
    def start_listening_for_request(self) :
        self.server.bind((self.host,self.port))
        self.server.listen()
        client , addr = self.server.accept()
        while self.running == True:
           # size_to_recv = int(client.recv(1024).decode("utf8"))
            #data = client.recv(size_to_recv).decode("utf8")
           # self.send_request(data,client)
           print("TRUE")

    #small program(don't need unittesting)
    def test_connection(self):
        self.server.bind((self.host,self.port))
        self.server.listen()
        client , addr = self.server.accept()
        print("successfully_connected")
        client.close()
        self.server.close()

    def test_basic_json_request(self):
        self.server.bind((self.host,self.port))
        self.server.listen()
        client , addr = self.server.accept()
        data = client.recv(1024).decode("utf8")

        try :
            data_dict = json.loads(data)
            password = data_dict["password"]
            command = data_dict["command"]
            print(f"expected password 'test' got : {password}")
            print(f"expected command 'test' got : {command}")
            client.send("Success".encode("utf8"))

        except:
            client.send("Error".encode("utf8"))



Middle().test_basic_json_request()
