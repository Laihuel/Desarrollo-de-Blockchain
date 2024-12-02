from brownie import accounts, config, network, VotationSystem, reverts
from scripts.utils import get_account
import pytest

@pytest.fixture
def votationSystem():
    #Arrange
    account = accounts[0]
    return VotationSystem.deploy({"from": account})

def test_votationSystem_deploy(votationSystem):
    # Act: Agregar un candidato con ID 1
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Verificar que el candidato fue agregado correctamente
    candidate = votationSystem.candidates(0)  # Obtenemos el primer candidato

    # Assert: Verificar que el ID del candidato es 1
    assert candidate[0] == 1, "El ID del candidato debería ser 1."
    assert candidate[1] == 0, "El número de votos del candidato debería ser 0."


def test_votationSystem_add_candidate_by_owner(votationSystem):
    # Act: Agregar un candidato con ID 1
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Verificar que el candidato fue agregado correctamente
    candidate = votationSystem.candidates(0)  # Obtenemos el primer candidato

    # Assert: Verificar que el ID del candidato es 1
    assert candidate[0] == 1, "El ID del candidato debería ser 1."
    assert candidate[1] == 0, "El número de votos del candidato debería ser 0."

def test_votationSystem_add_candidate_by_not_owner(votationSystem):
    # Act: Agregar un candidato con ID 1
    with reverts("Caller is not owner"):
        votationSystem.addCandidate(1, {"from": accounts[1]})


def test_add_voter_as_owner(votationSystem):
    # Act: Agregar una dirección a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Assert: Verificar que la dirección fue agregada a la whitelist
    assert votationSystem.whitelist(accounts[1]) == True, "La dirección debería estar en la whitelist."

def test_add_voter_as_non_owner(votationSystem):
    # Act & Assert: Intentar agregar una dirección a la whitelist desde una cuenta que no es owner
    with reverts("Caller is not owner"):
        votationSystem.addVoter(accounts[1], {"from": accounts[2]})  # accounts[2] no es owner


def test_voter_can_vote(votationSystem):
    # Act: Agregar un votante a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act: El votante (accounts[1]) emite su voto por el candidato con ID 1
    votationSystem.vote(1, {"from": accounts[1]})

    # Assert: Verificar que el candidato 1 recibió un voto
    candidate = votationSystem.candidates(0)  # Obtener el primer candidato
    assert candidate[1] == 1, "El candidato debería tener 1 voto."


def test_non_voter_cannot_vote(votationSystem):
    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act & Assert: Intentar votar desde una dirección que no está en la whitelist
    with reverts("Caller is not on the white list"):
        votationSystem.vote(1, {"from": accounts[2]})  # accounts[2] no está en la whitelist

def test_user_cannot_vote_multiple_times(votationSystem):
    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act: Agregar un votante a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Act: El votante (accounts[1]) vota por el candidato 1
    votationSystem.vote(1, {"from": accounts[1]})

    # Assert: Verificar que el votante (accounts[1]) no pueda votar otra vez
    with reverts("Caller has voted already"):
        votationSystem.vote(1, {"from": accounts[1]})  # El mismo votante intenta votar nuevamente


def test_owner_can_finish_votation(votationSystem):
    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act: Agregar un votante a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Act: El votante (accounts[1]) vota por el candidato 1
    votationSystem.vote(1, {"from": accounts[1]})

    # Act: El owner (accounts[0]) termina la votación
    votationSystem.finishVotation({"from": accounts[0]})

    # Assert: Verificar que la votación está terminada
    assert votationSystem.votationFinished() == True, "La votación debería estar marcada como finalizada."

def test_non_owner_cannot_finish_votation(votationSystem):
    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act: Agregar un votante a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Act: El votante (accounts[1]) vota por el candidato 1
    votationSystem.vote(1, {"from": accounts[1]})

    # Act & Assert: Intentar que una persona que no es owner (accounts[1]) termine la votación
    with reverts("Caller is not owner"):
        votationSystem.finishVotation({"from": accounts[1]})  # accounts[1] no es el owner

def test_cannot_vote_after_votation_ends(votationSystem):
    # Act: Agregar un candidato
    votationSystem.addCandidate(1, {"from": accounts[0]})

    # Act: Agregar un votante a la whitelist
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})

    # Act: El owner (accounts[0]) termina la votación
    votationSystem.finishVotation({"from": accounts[0]})

    # Act & Assert: Intentar que el votante (accounts[1]) vote después de que la votación terminó
    with reverts("Votation has finished already"):
        votationSystem.vote(1, {"from": accounts[1]})  # El votante ya no puede votar después de finalizar la votación
    
    #with reverts("Votation has finished already"):
        #votationSystem.vote(1, {"from": accounts[3]})  # El votante ya no puede votar después de finalizar la votación

def test_event_emission_on_finish_votation(votationSystem):
    # Arrange: Agregar un candidato y un votante
    votationSystem.addCandidate(1, {"from": accounts[0]})  # Agregar candidato con ID 1
    votationSystem.addVoter(accounts[1], {"from": accounts[0]})  # Agregar un votante a la whitelist
    votationSystem.vote(1, {"from": accounts[1]})  # El votante (accounts[1]) vota por el candidato 1

    # Act: Finalizar la votación
    tx = votationSystem.finishVotation({"from": accounts[0]})

    # Assert: Verificar que el evento 'VotationFinished' fue emitido
    assert "VotationFinished" in tx.events, "El evento VotationFinished no fue emitido."

    # Verificar que el ganador en el evento sea el esperado
    assert tx.events["VotationFinished"][0]["winner"] == 1, "El ganador del evento no es el esperado."

    # Verificar que el estado del contrato haya cambiado correctamente
    assert votationSystem.votationFinished() == True, "La votación debería estar marcada como finalizada."
    assert votationSystem.winner() == 1, "El ganador debería ser el candidato con ID 1."

def test_votation_finished_state(votationSystem):
    # Verificar el estado antes de finalizar la votación
    assert votationSystem.votationFinished() == False, "La votación no debería estar finalizada antes de llamarlo."

    # Terminar la votación
    votationSystem.finishVotation({"from": accounts[0]})

    # Verificar el estado después de finalizar la votación
    assert votationSystem.votationFinished() == True, "La votación debería estar finalizada después de llamarlo."

#----------------------------------------


def test_full_votation_flow(votationSystem):
    # Arrange: Configurar el escenario
    owner = accounts[0]
    voters = accounts[1:6]  # Usar 5 votantes
    candidate_ids = [1, 2, 3]

    # Agregar candidatos
    for candidate_id in candidate_ids:
        votationSystem.addCandidate(candidate_id, {"from": owner})

    # Agregar votantes a la whitelist
    for voter in voters:
        votationSystem.addVoter(voter, {"from": owner})

    # Emitir votos
    votationSystem.vote(1, {"from": voters[0]})  # Vota por candidato 1
    votationSystem.vote(2, {"from": voters[1]})  # Vota por candidato 2
    votationSystem.vote(2, {"from": voters[2]})  # Vota por candidato 2
    votationSystem.vote(3, {"from": voters[3]})  # Vota por candidato 3
    votationSystem.vote(3, {"from": voters[4]})  # Vota por candidato 3

    # Act: Finalizar la votación
    tx = votationSystem.finishVotation({"from": owner})

    # Assert: Verificar que la votación haya finalizado
    assert votationSystem.votationFinished() == True, "La votación debería estar marcada como finalizada."

    # Verificar que el ganador es el esperado (candidato 2)
    assert votationSystem.winner() == 2, "El candidato ganador debería ser el ID 2."

    # Verificar que el evento 'VotationFinished' fue emitido correctamente
    assert "VotationFinished" in tx.events, "El evento VotationFinished no fue emitido."
    assert tx.events["VotationFinished"][0]["winner"] == 2, "El ganador del evento no es el esperado."


