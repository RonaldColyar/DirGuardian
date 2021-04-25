

import asyncio
import websockets
import json
class Middle:

    def __init__(self):
        timeout_placeholder ={}
        timeout_placeholder["status"] = "failed"
        self.timeout_response = json.dumps(timeout_placeholder)
        
    def encrypt_request(self):
        pass
        
    
    def send_response_to_client(self ,server_response ):
            decrypted_response = self.decrypt_response(server_response)
        
    async def send_request(self ,request_dict ):
         async with websockets.connect('ws://localhost:8765') as websocket:

             server_greeting = await websocket.recv()
             json_request_string = json.dumps(request_dict)

             #send request and gather response.
             if self.decrypt_response(server_greeting) == "WHAT":
                 
                 encrypted_request  = self.encrypt_request(json_request_string)
                 await websocket.send(encrypted_request)
                 server_response = await websocket.recv() 
                
                 self.send_response_to_client(server_response , request_dict)

             else:
                 print(f"Server Responded with :{self.decrypt_response(server_greeting)}")
            

            
    def decrypt_response(self):
        pass
    