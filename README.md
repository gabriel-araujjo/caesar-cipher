# Caesar Cipher

A simple tool to encrypt and decrypt with messages using Caesar Cipher

```
Encrypt options:
  -k, --key <key>            - specifies a key to encrypting or decrypting process.
                                 The key must be a number between 1(including)
                                 and 25(including).
                                 Default: 3
  -o, --output <source_file> - sets the destiny output file.
                                 Default: stdout
  -i, --input <destny_file>  - sets the clear message source.
                                 Default: stdin
  -f, --force                - forces decryptation (brute force) and shows all
                                 possible results.
                                 Default: not force
Commands:
  encrypt                    - encrypts a message
  decrypt                    - decrypts a message

Usage:
  caesar encrypt [-k <key>] [-i <clear_text>] [-o <cipher>]
  caesar decrypt [-f] [-k <key>] [-i <cipher>] [-o <clear_text>]
```