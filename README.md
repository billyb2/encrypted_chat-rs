# Encrypted email chat (in Rust WASM)

This is an pretty basic encrypted email chat client written in Rust for WASM/

### How does it work?

Firstly, you need to add a recipient. To do this, you need them to send you your public key over a secure channel (so you know that the key hasn't been modified). You could do this in person, for example.

Then, once you've added the recipient, you just type in your message and click the "Encrypt" button. This will automatically open an email prompt with the encrypted message in the body, and the public key in the subject. 

When receiving a message, if you haven't added the recipient yet, it's recommended to do so by using the public key contained in the Subject of the message. Then, you simply copy and paste the received message into the "message" box, and click the decrypt button. If all goes well, you will now see their plaintext message.

The site keeps your private key in the localStorage of the webpage. This isn't the most secure method, and if your browser is compromised, it would be trivial to decyrpt any messages sent through email. If requested, I could add a way of encrypting your private key by prompting for a password whenever you go on the website.

The site uses the RustCrypto rsa library for encryption, as well as using the smaz compression algorithm (since it's good at compressing short strings, which messages usually are.) It also uses base64 to convert the encrypted binary data into something you can send over email.