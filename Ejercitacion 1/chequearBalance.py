from cryptos import Bitcoin

# Usar las palabras semilla originales
seed_words = 'spoil danger stomach boring champion arch cherry father kit shiver wife dragon'

# Configurar para testnet
coin = Bitcoin(testnet=True)

# Restaurar la billetera con las palabras clave (seed phrase)
wallet = coin.wallet(seed_words)

# Derivar la primera dirección de recepción explícitamente
addr1_restored = wallet.new_receiving_address()  # Crear y obtener la primera dirección de recepción

destino = "mz2R4owRMucX3euEUgV8FoGvEuw8fp8kni"  

print("Balance address 1: " + str(coin.get_balance(addr1_restored)))
print("Balance address 2: " + str(coin.get_balance(destino)))