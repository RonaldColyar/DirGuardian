
#converting to websockets for web usage
import asyncio
import websockets
import json
from TimeHandler import TimeTracker
class Server:
    def __init__(self):
        self.host = '127.0.0.1' #localhost
        self.port = 50222
        self.server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.running = True
        self.connected = 0
        self.server.bind((self.host,self.port))
        self.server.listen()

    def decrypt():
        pass
    def encrypt():
        pass
    def encryption_key(self):
        with open ("key.txt" , "r") as file:
           key =  file.readlines()
           return key

    #push notification
    def start_critical_sequence(self):
        pass

    def credentials_are_valid(self, message):
        if message == "TEST":
            return True
        elif message == "FAKEPASSWORDTRAP":
            self.start_critical_sequence()
            return False
        else:
            return False
            
    def check_command(self,client,message_dict):
        command = message_dict["password"]
        if command == "key":
            #key to decrypt directory
            encrypted_key = self.encrypt(self.encryption_key())
            client.send(encrypted_key.encode("ascii"))

        elif command == "report":
            pass

        else:
            pass
            
            





    def gather_and_check_declaration(self ,client ,addr):
        try:
            handler = TimeTracker().start_timer(client)
            client.send("WHAT".encode("ascii"))
            response = client.recv(100).decode("ascii")
            message_dict = json.loads(elf.decrypt(response))
            result = self.credentials_are_valid(message_dict["password"])
            if result == True:
                self.check_command(client,message_dict)
            else:
                print("Someone Tried to connect with wrong credentials")
                client.close()

                


        except:
            print("Client timeout!")

   
    def start_server(self):
        while self.running == True :
            client , addr = self.server.accept()
            self.connected += 1 
            thread = threading.Thread(target = self.gather_and_check_declaration, args=(client,addr))
            thread.start()

        