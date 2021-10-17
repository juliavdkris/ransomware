# The FileYeeter 3000
FileYeeter is a proof-of-concept ransomware for a school project. If you use this project for malicious purposes you're a dick, and a dumb dick at that. We did not implement any payment checks and we intentionally marched right into the classic ransomware pitfalls like not deleting shadow copies. There is plenty of ransomware out there, fuck off and go buy some shady code from hackforums.

## Design
FileYeeter, like most ransomware uses a hybrid encryption scheme. The "client" generates a symmetric key (ChaCha20) and an asymmetric key pair (x25519). The symmetric key is used to encrypt the victim's files, and is then encrypted using the client's public key. The server also generates an asymmetric key pair and sends its public key to the client. The client then uses the server's public key to encrypt their own private key, and stores the result. This way, the client needs the symmetric key to decrypt their files. To get this key, they have to send the encrypted version to the server, which then decrypts it with its private key and sends it back to the client.

## Installation
If for some reason you would like to ransomwarize yourself, here are the steps to do so:
1. Set up the [C&C server](https://github.com/juliavdkris/ransomware-c2) (simply set up the virtual environment and run c2.py)
2. Clone this repository
3. Create a `victim_files` folder in the project directory and throw some files in it
4. Compile and run the project with `cargo run` (or `cargo run decrypt` to decrypt the files). Alternatively, run `cargo build --release` to build the binary without running it.
5. Profit?

We would love to provide precompiled binaries for maximum convenience... But given the nature of this project, that sounds like a marvellously bad idea.
