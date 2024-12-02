from brownie import VotationSystem, accounts, network, config
from .utils import get_account

def deploy():
    account = get_account()
    VotationSystem.deploy({'from': account})

def verify():
    if len(VotationSystem) > 0:
        simple_counter = VotationSystem[-1]
        print(simple_counter)
        VotationSystem.publish_source(simple_counter)
    else:
        print("No se ha desplegado ningún contrato.")