use super::signature::Signature;

pub struct SignatureDb {
    signatures: Vec<Signature>,
}

impl SignatureDb {
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut db = Self::new();
        db.add(Signature::new(
            "EICAR-Test",
            "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR",
            "high",
        ));
        db.add(Signature::new(
            "Suspicious-Exec",
            "cmd.exe /c",
            "medium",
        ));
        db.add(Signature::new(
            "PowerShell-Download",
            "powershell -encodedcommand",
            "high",
        ));
        db.add(Signature::new(
            "Ransomware-Encrypt",
            "encrypt_all_files",
            "critical",
        ));
        db.add(Signature::new(
            "Keylogger-Hook",
            "SetWindowsHookEx",
            "high",
        ));
        db
    }

    pub fn add(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    pub fn scan(&self, payload: &str) -> Vec<&Signature> {
        self.signatures
            .iter()
            .filter(|sig| sig.matches(payload))
            .collect()
    }

    pub fn count(&self) -> usize {
        self.signatures.len()
    }
}
