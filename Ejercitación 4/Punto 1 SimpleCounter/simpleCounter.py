from brownie import SimpleCounter, accounts, network, config
from .utils import get_account

def deploy():
    account = get_account()
    simple_counter = SimpleCounter.deploy({'from': account})
    print(simple_counter.retrieveNumber())
    tx = simple_counter.increaseNumber({'from': account})
    tx.wait(1)
    print(simple_counter.retrieveNumber())

def verify():
    if len(SimpleCounter) > 0:
        simple_counter = SimpleCounter[-1]
        print(simple_counter)
        SimpleCounter.publish_source(simple_counter)
    else:
        print("No se ha desplegado ningún contrato.")
    