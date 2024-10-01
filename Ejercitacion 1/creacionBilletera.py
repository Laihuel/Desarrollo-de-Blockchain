from cryptos import entropy_to_words, Bitcoin
import os

# Generar una frase semilla
words = entropy_to_words(os.urandom(16))
print("mi frase semilla:", words)

# Configurar para testnet
coin = Bitcoin(testnet=True)
wallet = coin.wallet(words)

# Imprimir información del wallet
print("path de derivación:", wallet.keystore.root_derivation)
print("privada maestra:", wallet.keystore.xprv)
print("pública maestra:", wallet.keystore.xpub)

# Crear dirección de recepción
addr1 = wallet.new_receiving_address()
print("addr1:", addr1)
print("privada de addr1:", wallet.privkey(addr1))

# Crear segunda dirección de recepción
addr2 = wallet.new_change_address()
print("addr2:", addr2)
print("privada addr2:", wallet.privkey(addr2))
