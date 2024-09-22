import pexpect, os

dir=os.path.abspath(os.path.dirname(__file__))
rsa_key="rsa.key"
root_certificate="root_certificate.pem"
print("Creating the SSL key with openssl")
# Local CA
child = pexpect.spawn(f"openssl genrsa -des3 -out {dir}/{rsa_key} 2048")
child.expect("Enter PEM pass phrase:")
child.sendline("local")
child.expect("Verifying - Enter PEM pass phrase:")
child.sendline("local")
child.expect(pexpect.EOF)
print("RSA Key created succesfully")
# Root certificate
child = pexpect.spawn(f"openssl req -x509 -new -nodes -key {dir}/{rsa_key} -sha256 -days 1825 -out {dir}/{root_certificate}")
child.expect("Enter pass phrase for *")
child.sendline("local")
child.expect("Country Name (2 letter code)*")
child.sendline("FR")
child.expect("State or Province Name (full name)*")
child.sendline("Île-de-France")
child.expect("Locality Name (eg, city)*")
child.sendline("Paris")
child.expect("Organization Name (eg, company)*")
child.sendline("Mavio")
child.expect("Organizational Unit Name (eg, section)*")
child.sendline("Development")
child.expect("Common Name (e.g. server FQDN or YOUR name)*")
child.sendline("Local")
child.expect("Email Address*")
child.sendline("root@mavio.test")
child.expect(pexpect.EOF)
print("Root certificate created succesfully")