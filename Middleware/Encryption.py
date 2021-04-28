import socket
from cryptography.fernet import Fernet
class EncryptionHandler:

	def init_with_key():
		with open("privatekey.txt" , "rb" ) as file:
			self.key = file.read()
			self.f= Fernet(self.key)
	def make_key(self):
		key = Fernet.generate_key()
		self.key = key
		self.f = Fernet(self.key)
		with open("privatekey.txt", "wb") as file:
			file.write(key)
	def decrypted(self , message):
			return self.f.decrypt(message)
	def encrypted(self , message):
			return self.f.encrypt(message)
