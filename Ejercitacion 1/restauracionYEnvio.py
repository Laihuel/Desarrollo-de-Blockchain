from cryptos import Bitcoin

# Usar las palabras semilla originales
seed_words = 'spoil danger stomach boring champion arch cherry father kit shiver wife dragon'

# Configurar para testnet
coin = Bitcoin(testnet=True)

# Restaurar la billetera con las palabras clave (seed phrase)
wallet = coin.wallet(seed_words)

# Mostrar el camino de derivación para verificar que es el mismo
print("Camino de derivación restaurado:", wallet.keystore.root_derivation)

# Derivar la primera dirección de recepción explícitamente
addr1_restored = wallet.new_receiving_address()  # Crear y obtener la primera dirección de recepción
print("Primera dirección restaurada (addr1):", addr1_restored)

# Obtener la clave privada de esa dirección
privkey_addr1_restored = wallet.privkey(addr1_restored)
print("Clave privada de la primera dirección restaurada:", privkey_addr1_restored)




#Envío:
# Dirección de destino
destino = "mz2R4owRMucX3euEUgV8FoGvEuw8fp8kni"  

print("Balance address 1: " + str(coin.get_balance(addr1_restored)))
print("Balance address 2: " + str(coin.get_balance(destino)))

# Cantidad a enviar (en satoshis)
cantidad = 20

print("Enviando 20 satoshis de la primera dirección a la segunda...")
coin.send(privkey_addr1_restored, addr1_restored, destino, cantidad)
print("Enviado")

print("Balance address 1: " + str(coin.get_balance(addr1_restored)))
print("Balance address 2: " + str(coin.get_balance(destino)))
