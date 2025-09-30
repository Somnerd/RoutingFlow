import os, time, requests
BASE = os.environ.get("API_BASE", "http://api:8000")
def wait_for_api():
    for _ in range(60):
        try:
            r = requests.get(f"{BASE}/health", timeout=1.5)
            if r.status_code == 200:
                return True
        except Exception:
            pass
        time.sleep(1)
    return False
def test_health_endpoint():
    assert wait_for_api(), "API /health never became ready"
    r = requests.get(f"{BASE}/health", timeout=3)
    assert r.status_code == 200
    assert r.json().get("status") == "ok"
