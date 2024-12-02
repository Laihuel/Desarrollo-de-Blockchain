from brownie import accounts, config, network, SimpleCounter, reverts
from scripts.utils import get_account
import pytest

@pytest.fixture
def simpleCounter():
    #Arrange
    account = accounts[0]
    return SimpleCounter.deploy({"from": account})

def test_simpleCounter_deploy(simpleCounter):
    assert simpleCounter.retrieveNumber() == 0


#Chequear tema con este test. Teniendo el with reverts() pasa bien con brownie test, pero no pasa con bronie test --coverage.
#Sin el revert y con el assert es al reves, pasa con el coverage pero no con el test normal
"""
def test_retrieve_number_by_not_in_white_list(simpleCounter):
    non_whitelisted = accounts[1]
    #with reverts("Caller is not in the white list"):
    #    simpleCounter.retrieveNumber({"from": non_whitelisted})
    assert simpleCounter.retrieveNumber({"from": non_whitelisted}) == 0"""

def test_deployer_is_in_white_list(simpleCounter):
    deployer = accounts[0]
    assert simpleCounter.whiteList(deployer) == True, "Deployer should be in the whitelist"

def test_increase_number(simpleCounter):
    number = simpleCounter.retrieveNumber()
    simpleCounter.increaseNumber()
    number += 1
    assert simpleCounter.retrieveNumber() == number

def test_increase_number_not_in_white_list(simpleCounter):
    non_whitelisted = accounts[1]
    with reverts("Caller is not in the white list"):
        simpleCounter.increaseNumber({"from": non_whitelisted})

def test_decrease_number(simpleCounter):
    # Inicializa el contador a un valor mayor que 0
    simpleCounter.increaseNumber()  # Aumenta el contador a 1

    number = simpleCounter.retrieveNumber()
    simpleCounter.decreaseNumber()
    number -= 1
    assert simpleCounter.retrieveNumber() == number

def test_decrease_number_when_zero(simpleCounter):
    # Asegurarse de que el contador está en 0
    assert simpleCounter.retrieveNumber() == 0
    
    # Verifica que no se pueda decrementar más
    with reverts("Counter cannot be negative"):
        simpleCounter.decreaseNumber()


def test_add_to_white_list_by_owner(simpleCounter):
    # Arrange
    owner = accounts[0]  # El propietario
    address_to_add = accounts[1]  # Dirección que se va a añadir

    # Act
    simpleCounter.addToWhiteList(address_to_add, {"from": owner})

    # Assert
    assert simpleCounter.whiteList(address_to_add) == True, "Address was not added to whitelist"

def test_add_to_white_list_by_non_owner(simpleCounter):
    # Arrange
    owner = accounts[0]  # El propietario
    non_owner = accounts[1]  # No propietario
    address_to_add = accounts[2]  # Dirección que se va a intentar añadir

    # Act & Assert
    with reverts("Caller is not owner"):  # Se espera que se lance un error porque no es propietario
        simpleCounter.addToWhiteList(address_to_add, {"from": non_owner})

def test_remove_from_white_list(simpleCounter):
    # Arrange
    owner = accounts[0]  # El propietario
    non_owner = accounts[1]  # No propietario
    address_to_add = accounts[2]  # Dirección que vamos a añadir y luego quitar

    # Act: Añadir la dirección a la whitelist
    simpleCounter.addToWhiteList(address_to_add, {"from": owner})

    # Assert: Verificar que la dirección fue añadida
    assert simpleCounter.whiteList(address_to_add) == True

    # Act: Intentar eliminar la dirección como propietario
    simpleCounter.removeFromWhiteList(address_to_add, {"from": owner})

    # Assert: Verificar que la dirección fue eliminada
    assert simpleCounter.whiteList(address_to_add) == False

def test_remove_from_white_list_by_non_owner(simpleCounter):
    # Arrange
    owner = accounts[0]  # El propietario
    non_owner = accounts[1]  # No propietario
    address_to_add = accounts[2]  # Dirección que vamos a intentar quitar

    # Act: Añadir la dirección a la whitelist
    simpleCounter.addToWhiteList(address_to_add, {"from": owner})

    # Assert: Verificar que la dirección fue añadida
    assert simpleCounter.whiteList(address_to_add) == True

    # Act & Assert: Intentar eliminar la dirección como no propietario
    with reverts("Caller is not owner"):  # Se espera que lance un error
        simpleCounter.removeFromWhiteList(address_to_add, {"from": non_owner})

def test_increase_number_emits_event(simpleCounter):
    initial_number = simpleCounter.retrieveNumber()
    tx = simpleCounter.increaseNumber()
    
    # Verifica que se emita el evento NewValue con el valor correcto
    assert tx.events["NewValue"]["newNumber"] == initial_number + 1


def test_decrease_number_emits_event(simpleCounter):
    simpleCounter.increaseNumber()
    simpleCounter.increaseNumber()
    initial_number = simpleCounter.retrieveNumber()
    tx = simpleCounter.decreaseNumber()
    
    # Verifica que se emita el evento NewValue con el valor correcto
    assert tx.events["NewValue"]["newNumber"] == initial_number - 1

@pytest.mark.parametrize("action, expected_value", [
    ("increase", 1),
    ("decrease", 0),
])
def test_counter_actions(simpleCounter, action, expected_value):
    if action == "increase":
        simpleCounter.increaseNumber()
    elif action == "decrease":
        simpleCounter.increaseNumber()
        simpleCounter.decreaseNumber()
    
    assert simpleCounter.retrieveNumber() == expected_value

