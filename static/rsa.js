export async function fetchRsaKey() {
    try {
        const response = await fetch('/rsa/key');

        if (!response.ok) {
            throw new Error(`Failed to fetch RSA key. Status: ${response.status}`);
        }
        const keyData = await response.json();
        console.log("Got: ", keyData);

        const e = new forge.jsbn.BigInteger(keyData.e);
        const n = new forge.jsbn.BigInteger(keyData.n);
        console.log("e is: ", e);
        console.log("n is: ", n);

        // Create a Forge RSA public key
        const rsaPublicKey = forge.pki.setRsaPublicKey(n, e);

        return rsaPublicKey;
    } catch (error) {
        console.error('Error fetching RSA key:', error);
        throw error; // You may want to handle this error in your application
    }
}

// Example function to generate an RSA key pair
function generateKeyPair() {
    const keyPair = forge.pki.rsa.generateKeyPair({ bits: 2048 });

    return keyPair;
}
function encryptWithPublicKey(publicKey, message) {
    console.log("Encrypting");

    // Convert the UTF-8 string to a byte array
    const messageBytes = forge.util.encodeUtf8(message);

    // Encrypt the byte array using RSAES-PKCS1-V1_5 padding
    const encryptedBytes = publicKey.encrypt(messageBytes, 'RSAES-PKCS1-V1_5');

    console.log("Encrypted Bytes is: ", encryptedBytes);

    // Convert the encrypted bytes to a Uint8Array
    const encryptedArray = forge.util.createBuffer(encryptedBytes).getBytes();

    return encryptedArray;
}

// Example function to decrypt a message with RSA private key using an array of bytes
function decryptWithPrivateKey(privateKey, encryptedBytes) {
    // Convert encryptedBytes to a Forge buffer
    const encryptedBuffer = forge.util.createBuffer(encryptedBytes);

    // Decrypt using the private key
    const decrypted = privateKey.decrypt(encryptedBuffer.getBytes(), 'RSAES-PKCS1-V1_5');

    // Convert the decrypted data to a UTF-8 string
    return forge.util.decodeUtf8(decrypted);
}



// Function to send encrypted data to the server
async function sendEncryptedData(encryptedDataObject, url) {
    try {
        console.log("Sending: ", encryptedDataObject);
        console.log("Sending (JSON): ", JSON.stringify(encryptedDataObject));
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(encryptedDataObject),
        });

        if (!response.ok) {
            let body = await response.text();
            throw new Error(`Failed to send encrypted data. Status: ${response.status}, Body: ${body}`);
        }

        console.log('Encrypted data sent successfully');
    } catch (error) {
        console.error('Error sending encrypted data:', error);
        throw error; // You may want to handle this error in your application
    }
}

export async function example() {
    // Example usage
    const keyPair = generateKeyPair();

    // Assume you have a public key and a private key
    const publicKey = keyPair.publicKey;
    const privateKey = keyPair.privateKey;

    const message = "test message";

    // Encrypt with the public key
    const encryptedMessage = encryptWithPublicKey(publicKey, message);
    console.log("Encrypted Message:", encryptedMessage);

    // Decrypt with the private key
    const decryptedMessage = decryptWithPrivateKey(privateKey, encryptedMessage);
    console.log("Decrypted Message:", decryptedMessage);
}

function hexStringToBytes(hexString) {
    // Check if the input is a valid hexadecimal string
    if (!/^[0-9A-Fa-f]+$/.test(hexString)) {
        throw new Error("Invalid hexadecimal string");
    }

    // Split the string into pairs of two characters
    const pairs = hexString.match(/.{1,2}/g);

    // Convert pairs to an array of bytes
    const bytes = pairs.map(pair => parseInt(pair, 16));

    return bytes;
}

export async function encryptSend() {
    const publicKey = await fetchRsaKey();
    var message = document.getElementById('rsaText').value;
    // var message = "test message";
    const encryptedMessage = encryptWithPublicKey(publicKey, message);
    // Create a TextEncoder instance
    const textEncoder = new TextEncoder();
    // Convert the string to Uint8Array
    const uint8Array = textEncoder.encode(encryptedMessage);
    const sentMessage = {
        data: Array.from(uint8Array)
    };
    await sendEncryptedData(sentMessage, '/rsa/');
}
