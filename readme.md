# Crust

todo...

## Passwords

- Passwords must be hashed using one of the [cryptographically secure hashing algorithms](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#password-hashing-algorithms).
- Passwords must be [salted](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#salting). Minimum salt length is [16 bytes](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html#name-storage-2).
- Passwords should be [peppered](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#peppering). Minimum pepper length is [32 bytes](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html#name-storage-2). If a pepper is used, consideration should be taken to ensure that it can be easily [rotated](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html#name-storage-2:~:text=If%20a%20pepper%20is%20used%2C%20consideration%20should%20be%20taken%20to%20ensure%20that%20it%20can%20be%20easily%20rotated.).
- [The work factor of the hashing algorithm should be increased as the performance of the hardware increases](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#upgrading-the-work-factor).
- The minimum length of the password should be set to [8 characters](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html#name-storage-2:~:text=minimum%20length%20of-,8%20characters,-for%20user%20passwords) and the maximum length of the password should be set to [64 characters](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html#name-storage-2:~:text=64%20and%20128%20characters) to prevent denial of service (DoS) attacks.
- Hashing algorithm must support [international characters](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#international-characters).

### Resources

- [Password Storage Cheat Sheet.](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Best practices for password hashing and storage.](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-04.html)
