
#converting to websockets for web usage
import asyncio
import websockets
import json
from TimeHandler import TimeTracker
from Encryption import EncryptionHandler
class Server:
    def __init__(self):
        self.host = '127.0.0.1' #localhost
        self.port = 50222
        self.connected = 0
        self.Eh = EncryptionHandler()
  
    def dir_encryption_key(self):
        with open ("key.txt" , "r") as file:
           key =  file.readlines()
           return key

    #push notification or email
    def start_critical_sequence(self):
        pass
    
    def fake_password():
        with open("fakepassword.txt" , "r") as file:
            fake = file.readlines()
            return fake
    def password(self):
        with open("password.txt" , "r") as file:
            password = file.readlines()
            return password

    def credentials_are_valid(self, message):
        if message == self.password():
            return True
        elif message == self.fake_password():
            self.start_critical_sequence()
            return False
        else:
            return False
            
    async def check_command(self,websocket,message_dict):
        command = message_dict["command"]
        if command == "key":
            #encrypts the directory encryption key with the communication encryption key
            encrypted_key = self.Eh.encrypted(self.dir_encryption_key())
            await websocket.send(encrypted_key)

        elif command == "report":
            pass

        else:
            pass
            
            





    async def gather_and_check_declaration(self ,websocket,path):
        try:
            handler = TimeTracker().start_timer(websocket)
            await websocket.send("WHAT")
            response  = await websocket.recv()

            message_dict = json.loads(self.Eh.decrypted(response))
            result = self.credentials_are_valid(message_dict["password"])
            if result == True:
                self.check_command(websocket,message_dict)
            else:
                print("Someone Tried to connect with wrong credentials")
                websocket.close()

        except:
            websocket.close()
            print("Client timeout!")

   
    def start_server(self):
           start_server = websockets.serve(self.gather_and_check_declaration, self.host, self.port)
           asyncio.get_event_loop().run_until_complete(start_server)
           asyncio.get_event_loop().run_forever()



if __name__ == "__main__":
    server = Server()
    server.start_server()