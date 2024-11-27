"use client";

import { useState } from "react";

const GenerateKeys = () => {
  const [keys, setKeys] = useState({ publicKey: "", privateKey: "" });

  const generateKeyPair = async () => {
    try {
      const response = await fetch("http://localhost:8000/generate_key_pair");
      const data = await response.json();
      console.log(data);
      setKeys({ publicKey: data[0], privateKey: data[1] });
    } catch (error) {
      console.error("Error fetching keys:", error);
    }
  };

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1>Generate Keys</h1>
      <button onClick={generateKeyPair}>Generate</button>
      {keys.publicKey && keys.privateKey && (
        <div>
          <p>
            <strong>Public Key:</strong> {keys.publicKey}
          </p>
          <p>
            <strong>Private Key:</strong> {keys.privateKey}
          </p>
        </div>
      )}
    </div>
  );
};

export default GenerateKeys;
