[CryptoHack – General challenges](https://cryptohack.org/challenges/general/)

> PEM is a popular format for sending and receiving keys, certificates, and other cryptographic material. It looks like:
>
>       \-----BEGIN RSA PUBLIC KEY-----  
>       MIIBCgKC... _(a whole bunch of base64)_  
>       \-----END RSA PUBLIC KEY-----
>
> It wraps base64-encoded data by a one-line header and footer to indicate how to parse the data within. Perhaps unexpectedly, it's important for there to be the correct number of hyphens in the header and footer, otherwise cryptographic tools won't be able to recognise the file.
>
> The data that gets base64-encoded is DER-encoded ASN.1 values. Confused? [Here](https://www.cryptologie.net/article/260/asn1-vs-der-vs-pem-vs-x509-vs-pkcs7-vs/) is more information about what these acronyms mean but the complexity is there for historical reasons and going too deep into the details may drive you insane.
>
> Extract the private key _d_ as a decimal integer from this PEM-formatted RSA key.
>
> [privacy\_enhanced\_mail.pem](https://cryptohack.org/static/challenges/privacy_enhanced_mail_1f696c053d76a78c2c531bb013a92d4a.pem)
