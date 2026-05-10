import network
import time
import ubinascii

SSID = "PICO-SETUP"
PASSWORD = "pico1234"

ap = network.WLAN(network.AP_IF)

def start_ap():
    ap.config(
        essid=SSID,
        password=PASSWORD
    )
    ap.active(True)

    while not ap.active():
        time.sleep(0.1)

    print("AP started")
    print("IP:", ap.ifconfig()[0])


def wait_for_client(poll_interval=0.5):
    print("Waiting for client...")
    while True:
        try:
            stations = ap.status("stations")
            if stations:
                print("Client connected")
                # return formatted client info
                return get_connected_clients()
        except:
            pass
        time.sleep(poll_interval)

def _format_mac(mac_bytes):
    return ':'.join('{:02X}'.format(b) for b in mac_bytes)

def get_connected_clients():
    """
    Returns a list of dicts with keys: mac, ip, rssi (rssi may be None).
    """
    try:
        stations = ap.status("stations")
    except Exception:
        return []

    clients = []
    for s in stations: # type: ignore
        # station tuple layout can vary; commonly (mac, ip) or (mac, ip, rssi)
        mac = None
        ip = None
        rssi = None

        if len(s) >= 1 and s[0]:
            mac = _format_mac(s[0])
        if len(s) >= 2:
            ip = s[1]
        if len(s) >= 3:
            rssi = s[2]

        clients.append({"mac": mac, "ip": ip, "rssi": rssi})
    return clients
