import requests
import datetime

BINANCE_URL_SPOT = 'https://api.binance.com/api/v3'
BINANCE_URL_PERP = 'https://fapi.binance.com/fapi/v1'


def all_spot_pairs():
    res = requests.get(BINANCE_URL_SPOT + "/exchangeInfo")
    if not res.status_code == 200:
        print(f"Something went wrong: {res.status_code} {res.reason}")
    return [{'symbol': sym['symbol'], 'baseAsset': sym['baseAsset'], 'quoteAsset': sym['quoteAsset']} for sym in res.json()['symbols'] if sym['status'] == 'TRADING']


def get_spot_ohlcv(symbol, interval, limit, endtime):
    res = requests.get(BINANCE_URL_SPOT + f"/klines?symbol={symbol}&interval={interval}&limit={limit}&endTime={endtime}")
    if not res.status_code == 200:
        print(f"Something went wrong: {res.status_code} {res.reason}")
    return res.json()


def scrape_all_ohlcv(symbol, interval):
    ret = []
    res = get_spot_ohlcv(symbol=symbol, interval=interval, limit=1000, endtime=int(datetime.datetime.now().timestamp() * 1000))
    ret.extend(res[:-1])
    while len(res) == 1000:
        res = get_spot_ohlcv(symbol=symbol, interval=interval, limit=1000, endtime=res[0][0])
        ret.extend(res[:-1]) 
    return ret 

