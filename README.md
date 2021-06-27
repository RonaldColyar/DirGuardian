# DirGuardian

note: Remote functionality incomplete.Simple to implement , but not needed for my use cases at this point in time.

A System that protects a selected directory with cloud functionality.

- Encrypt a directory and deceive attackers who try to access this directory.
- Remove access to the encryption key from the local device and store it remotely
- Custom breach protocols
- Customizable interface
- offline mode
- Dead mans switch
- Guardian teams
- Real-time notifications on dir statuses
- Directory mirroring



# Setup( detailed instructions on how included in docs):
1. You must gather a Fernet encryption key for your client and server( ONLY FOR SECURE COMMUNICATIONS)
2. Place this Fernet encryption key in the environment of both the client and server(.txt file)
3. You must place a personal access password in the environment of the server(for authentication).
4. Configure the middleware application to the address of your hosted server.
5. Start middleware
6. Start client
7. Begin running the cli commands(since the server should already be running externally)

# why use the middleware?

1.test of interfacing rust socket connection with python's implementation.

# How does this system work?

### (Note: Every request to your server requires the password in step 3 above)
### Dummy explanation
1.You encrypt your directory

2.Key used to encrypt is stored on your server

3.When it is time to decrypt you need to request this key with a configured password

4.Client decrypts with the key

### More in depth
1. The user of this system must use the rust client to encrypt a local directory/sub directories(with a randomly/manually generated encryption key)
- If this is online encryption then the middleware forwards the key to the server and doesn't keep it locally
- If this is offline encryption then the middleware isn't needed and you store the key locally(or memorize it)
2. To decrypt a directory that was encrypted with online encryption the client requests the key from the server.
- On gather of the key ,the client then decrypts the directory.
3. To decrypt a directory that was encrypted locally you will just need to provide the client with the key.


# Is there  only command line access???

There is will be a matching web and android version to monitor the dir status and control the server's state. 


# Sample Interface
<img src="/ui.png">
