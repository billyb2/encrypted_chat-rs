      import init, { enc, gen_pub_key, gen_priv_key, dec, b64_encode, b64_decode } from "./enc_chat.js";
      
      await init();

      let log = document.getElementById("log");
      log.innerHTML = "Welcome!";

      if (localStorage.getItem("recipient") !== null) {
        document.getElementById("recipient").value = localStorage.getItem("recipient");

      }

      if (localStorage.getItem("priv_key") === null) {
        // Generate a new private key
        log.innerHTML = "Generating new keypair";

        let priv_key = gen_priv_key();
        localStorage.setItem("priv_key", priv_key);
      } 

      let priv_key = localStorage.getItem("priv_key");
      let pub_key = gen_pub_key(priv_key);

      function encrypt_and_send(message) {
        if (localStorage.getItem(document.getElementById("recipient").value) != null ) {
          log.innerHTML = "Encrypting message...";
          let enc_result = enc(message, localStorage.getItem(document.getElementById("recipient").value));

          localStorage.setItem("recipient", document.getElementById("recipient").value);
          log.innerHTML = "Encrypted and sent message!";
          
          window.open("mailto:" + document.getElementById("recipient").value + "?subject=" + b64_encode(pub_key) + "&body=" + enc_result);
        } else {
          log.innerHTML = "Recipient public key not found! Please click the \"Add Recipient\" button below and paste the recipient's public key (found in the subject of their email message)";

        }
      }

      function decrypt(message) {
        log.innerHTML = "Decrypting message...";
        let dec_result = dec(message, localStorage.getItem("priv_key"));

        log.innerHTML = "Message received: " + dec_result;

      }

      function add_recipient() {
        let email = document.getElementById("recipient").value;
        let pub_key = prompt("Recipient public key (found in subject of email):");

        if (email == "" || email == null || pub_key == null || pub_key == "") {
          log.innerHTML = "Invalid email or public key";

        } else {
          localStorage.setItem(email, b64_decode(pub_key));
          alert(localStorage.getItem(email));

        }

        log.innerHTML = "";
      }

      function clear_data() {
        localStorage.clear();
        log.innerHTML = "Data cleared successfully! Please refresh the page";
        
      }

      document.getElementById("send_message").onclick = function() { encrypt_and_send(document.getElementById("message").value); };
      document.getElementById("decrypt_message").onclick = function() { decrypt(document.getElementById("message").value); };
      document.getElementById("add_recipient").onclick = function() { add_recipient(); };
      document.getElementById("clear_data").onclick = function() { clear_data(); };
