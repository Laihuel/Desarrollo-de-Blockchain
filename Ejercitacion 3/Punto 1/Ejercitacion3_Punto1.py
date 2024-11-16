from web3 import Web3
import os
import json
import getpass  # Para pedir la clave privada de forma segura

# Dirección del contrato y ABI
CONTRACT_ADDRESS = "0x1f3569c66b9a8ab114a49d85229970e57e676211"
# Convertir a formato checksum
CHECKSUM_ADDRESS = Web3.to_checksum_address(CONTRACT_ADDRESS)
ABI = [
    {
        "inputs": [],
        "stateMutability": "nonpayable",
        "type": "constructor"
    },
    {
        "anonymous": False,
        "inputs": [
            {"indexed": True, "internalType": "address", "name": "sender", "type": "address"},
            {"indexed": False, "internalType": "uint256", "name": "newNumber", "type": "uint256"}
        ],
        "name": "NewValue",
        "type": "event"
    },
    {
        "inputs": [{"internalType": "address", "name": "_address", "type": "address"}],
        "name": "addToWhiteList",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [{"internalType": "address", "name": "_address", "type": "address"}],
        "name": "removeFromWhiteList",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "increaseNumber",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "decreaseNumber",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "retrieveNumber",
        "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [{"internalType": "address", "name": "", "type": "address"}],
        "name": "whiteList",
        "outputs": [{"internalType": "bool", "name": "", "type": "bool"}],
        "stateMutability": "view",
        "type": "function"
    }
]

# Conecta al nodo
#NODE = os.environ.get("NODE")
NODE = "https://sepolia.infura.io/v3/de69aef06d794ec7bba8ef870e13acfa"
if not NODE:
    print("La variable de entorno NODE no está configurada")
    exit()
w3 = Web3(Web3.HTTPProvider(NODE))
w3.is_connected()


# Configuración del contrato
contract = w3.eth.contract(address=CHECKSUM_ADDRESS, abi=ABI)

# Dirección de la cuenta desde la que interactuarás
# Solicita la dirección desde la terminal
MY_ADDRESS = input("Introduce tu clave pública (dirección de la cuenta): ").strip()
# Validación básica de la dirección
if not Web3.is_address(MY_ADDRESS):
    print("La dirección ingresada no es válida.")
    exit()

print(f"Usando la dirección: {MY_ADDRESS}")


# Función para leer información del contrato
def read_contract():
    number = contract.functions.retrieveNumber().call({'from': MY_ADDRESS})
    print(f"El número actual es: {number}")


# Función para escribir en el contrato
def write_contract(func_name, private_key, *args):
    # Construye la transacción
    nonce = w3.eth.get_transaction_count(MY_ADDRESS)
    tx = contract.functions[func_name](*args).build_transaction({
        'from': MY_ADDRESS,
        'nonce': nonce,
        'gas': 2000000,
        'gasPrice': w3.to_wei('20', 'gwei')
    })
    # Firma la transacción
    signed_tx = w3.eth.account.sign_transaction(tx, private_key)
    # Envia la transacción
    tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)
    print(f"Transacción enviada: {w3.to_hex(tx_hash)}")


# Ejecución
if __name__ == "__main__":
    # Pide la clave privada por terminal
    PRIVATE_KEY = getpass.getpass("Introduce tu clave privada: ").strip()

    # Asegúrate de que la clave privada es válida
    if not PRIVATE_KEY or len(PRIVATE_KEY) != 64:
        print("La clave privada no es válida.")
        exit()



    # Lee el número actual
    read_contract()

    # Incrementa el número (por ejemplo)
    write_contract("increaseNumber", PRIVATE_KEY)
