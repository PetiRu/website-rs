export type Challenge = Readonly<{
  token: string;
  prompt: string;
  expiresInSeconds: number;
}>;

export type SecurityHeaders = Readonly<Record<string, string>>;

export function securityHeaders(): SecurityHeaders {
  return {
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
    "Referrer-Policy": "strict-origin-when-cross-origin",
    "Content-Security-Policy": "default-src 'self'",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
  };
}

export class MemoryCaptcha {
  #pending = new Map<string, { answer: string; expiresAt: number }>();

  issueMath(ttlSeconds = 120): Challenge {
    const left = Math.floor(Math.random() * 18) + 2;
    const right = Math.floor(Math.random() * 18) + 2;
    const token = crypto.randomUUID();
    this.#pending.set(token, { answer: String(left + right), expiresAt: Date.now() + ttlSeconds * 1000 });
    return { token, prompt: `What is ${left} + ${right}?`, expiresInSeconds: ttlSeconds };
  }

  verify(token: string, answer: string): boolean {
    const pending = this.#pending.get(token);
    this.#pending.delete(token);
    return Boolean(pending && pending.expiresAt > Date.now() && pending.answer === answer.trim());
  }
}

export async function encryptAesGcm(key: CryptoKey, plaintext: Uint8Array, associatedData?: Uint8Array): Promise<Uint8Array> {
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const ciphertext = new Uint8Array(await crypto.subtle.encrypt({ name: "AES-GCM", iv, additionalData: associatedData }, key, plaintext));
  const output = new Uint8Array(iv.length + ciphertext.length);
  output.set(iv);
  output.set(ciphertext, iv.length);
  return output;
}
