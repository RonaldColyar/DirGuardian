class Server:
    def __init__(self):
        def __init__(self):
        self.host = '127.0.0.1' #localhost
        self.port = 50222
        self.server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.running = True
        self.connected = 0
        self.server.bind((self.host,self.port))
        self.server.listen()
    def start_server(self):
        while self.running == True :
            client , addr = self.server.accept()
            self.connected += 1 
            thread = threading.Thread(target = self.check_declaration , args=(client,addr))
            thread.start()

        