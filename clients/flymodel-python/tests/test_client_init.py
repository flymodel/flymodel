from flymodel import Client


# smoke test
def test_client_init():
    _ = Client(base_url="http://localhost:9009")
