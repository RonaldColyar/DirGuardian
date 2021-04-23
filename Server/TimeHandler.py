
import time
import threading

class TimeTracker:
	def __init__(self):
        self.running = True
    def check_elasped_time(self ,client):
        current_time = time.time()
        #only can be connected to the proxy
        #without stating who you are for 20 seconds!
        while self.running == True:
            elasped_time = int(current_time -clock.start_time)
            if elasped_time >20 :
                client.close()
                self.stop_timer()
    def stop_timer(self ):
            self.running = False

    def start_timer(self , client,num):
        thread = threading.Thread(target = self.monitor_timeout , args = (client,))
        thread.start()

    def monitor_timeout(self, client ):
            self.check_elasped_time(client)
