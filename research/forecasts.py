import psycopg2
import pandas as pd
import numpy as np
import pandas_ta as ta
import matplotlib.pyplot as plt
import statsmodels.api as sm 
from scipy.stats import pearsonr

conn = psycopg2.connect("postgres://postgres:postgres@192.168.8.105/trendfollowing")
cur = conn.cursor()

query_filtered = """SELECT pf.symbol, to_timestamp(o.opentime / 1000) AS opentime, o.closeprice::decimal
FROM daily_ohlcv o
JOIN perpetual_futures pf ON pf.id = o.coin_id
WHERE pf.symbol LIKE '%USDT'
AND o.coin_id IN (
    SELECT coin_id
    FROM daily_ohlcv
    GROUP BY coin_id
    HAVING COUNT(*) >= 365
)
ORDER BY o.opentime ASC;
"""

query_raw = "SELECT pf.symbol, to_timestamp(o.opentime / 1000) as opentime, o.closeprice::decimal FROM daily_ohlcv o JOIN perpetual_futures pf ON pf.id = o.coin_id WHERE symbol LIKE '%USDT' ORDER BY o.opentime ASC"

def load_df():
    cur.execute(query_filtered)
    perps = cur.fetchall()
    df = pd.DataFrame(perps)
    df.columns = ["symbol", "opentime", "close"]
    df['close'] = df['close'].astype(float)
    df = df.pivot(columns="symbol", values="close", index="opentime")
    return df

def bollinger_momentum(closes, index, lookback, stdv):
    df = pd.DataFrame()
    df['close'] = closes
    df.index = index
    bands = df.ta.bbands(lookback, stdv)
    df['bbu'] = bands[f'BBU_{lookback}_{stdv}.0']
    df['bbl'] = bands[f'BBL_{lookback}_{stdv}.0']
    df['forecast'] = ((df['close'] - df['bbl'] )/(df['bbu'] - df['bbl'])) * stdv - 1

    retdf = pd.DataFrame()
    retdf.index = df.index
    retdf[f'forecast'] = (df['forecast'] * 10).clip(lower=-20, upper=20)


    return retdf


def breakouts(closes, index, lookback):
    df = pd.DataFrame()
    df['close'] = closes
    df.index = index
    df['bbu'] = df['close'].rolling(lookback).max()
    df['bbl'] = df['close'].rolling(lookback).min()
    df['forecast'] = ((df['close'] - df['bbl'] )/(df['bbu'] - df['bbl'])) * 2 - 1

    retdf = pd.DataFrame()
    retdf.index = df.index
    retdf[f'forecast'] = df['forecast'] * 20

    return retdf


def ema_cross(closes, index, fast, slow):
    df = pd.DataFrame()
    df['close'] = closes
    df.index = index
    df['fast'] = df.ta.ema(fast)
    df['slow'] = df.ta.ema(slow)
    df['forecast'] = df['fast'] - df['slow'] 
    df['adjusted_forecast'] = (df['forecast'] * 10) / (df['forecast'].abs().mean())
    df['clamped_forecast'] = df['adjusted_forecast'].clip(upper=20, lower=-20)

    retdf = pd.DataFrame()
    retdf.index = df.index
    retdf[f'forecast'] = df['clamped_forecast']

    return retdf


