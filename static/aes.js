// Converte os bytes de uma chave para uma chave aes.
async function importKeyFromArrayBuffer(keyBytes) {
    const key = await crypto.subtle.importKey(
        'raw',          // format
        keyBytes,       // key data as ArrayBuffer
        { name: 'AES-GCM' },
        false,          // extractable
        ['encrypt', 'decrypt']
    );

    return key;
}

// Coleta a chave AES em /aes/key, converte para uma chave AES, e retorna.
async function fetchAesKey() {
    try {
        const response = await fetch('/aes/key');

        if (!response.ok) {
            throw new Error(`Failed to fetch AES key. Status: ${response.status}`);
        }

        const keyData = await response.json();

        // Assuming keyData.key is an array of 32 bytes in another format (e.g., an array of numbers)
        const keyBytes = keyData.key;

        // Convert to Uint8Array
        const uint8ArrayKey = new Uint8Array(keyBytes);

        // Assuming keyData.key is an array of 32 bytes (Uint8Array)
        const aesKey = await importKeyFromArrayBuffer(uint8ArrayKey);

        return aesKey;
    } catch (error) {
        console.error('Error fetching AES key:', error);
        throw error; // You may want to handle this error in your application
    }
}

// Criptografa uma mensagem de acordo com uma chave AES
async function encryptMessage(key, message) {
    const nonce = crypto.getRandomValues(new Uint8Array(12));
    const encodedMessage = new TextEncoder().encode(message);

    const encryptedData = await crypto.subtle.encrypt(
        {
            name: 'AES-GCM',
            iv: nonce,
        },
        key,
        encodedMessage
    );

    return {
        data: new Uint8Array(encryptedData),
        nonce: nonce,
    };
}

// Descriptografa uma mensagem de acordo com uma chave AES.
async function decryptMessage(key, encryptedData, nonce) {
    const decryptedData = await crypto.subtle.decrypt(
        {
            name: 'AES-GCM',
            iv: nonce,
        },
        key,
        encryptedData
    );

    return new TextDecoder().decode(decryptedData);
}

// Função que envia a mensagem criptografada para o servidor.
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

// Coleta a chave AES, coleta o texto no componente 'aesText', criptografa o texto, envia para o servidor.
export default async function sendAesMessage() {
    try {
        const aesKey = await fetchAesKey();

        // Get the text input value
        var plaintext = document.getElementById('aesText').value;
        const encrypted = await encryptMessage(aesKey, plaintext);

        // Create JSON object with encrypted data and nonce
        const encryptedDataObject = {
            data: Array.from(encrypted.data),
            nonce: Array.from(encrypted.nonce),
        };

        // Send the encrypted data to Rust server
        await sendEncryptedData(encryptedDataObject, '/aes/');

    } catch (error) {
        // Handle errors here
        console.error('Example usage error:', error);
    }
}





















