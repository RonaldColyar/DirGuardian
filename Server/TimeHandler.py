
import time
import threading

class TimeTracker:
    def __init__(self):
        self.running = True
    def check_elasped_time(self ,websocket):
        current_time = time.time(websocket)
        #only can be connected to the proxy
        #without stating who you are for 20 seconds!
        while self.running == True:
            elasped_time = int(current_time -clock.start_time)
            if elasped_time >20 :
                websocket.close()
                self.stop_timer()
    def stop_timer(self ):
            self.running = False

    def start_timer(self , websocket):
        thread = threading.Thread(target = self.monitor_timeout , args = (websocket,))
        thread.start()

    def monitor_timeout(self, websocket ):
            self.check_elasped_time(websocket)
