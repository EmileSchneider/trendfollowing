import psycopg2
import pandas as pd
import numpy as np
import pandas_ta as ta
import matplotlib.pyplot as plt
import statsmodels.api as sm 
from scipy.stats import pearsonr


CONN = psycopg2.connect("postgres://postgres:postgres@192.168.8.105/trendfollowing")
CUR = CONN.cursor()


def load_df():
    CUR.execute("SELECT pf.symbol, to_timestamp(o.opentime / 1000) as opentime, o.closeprice::decimal  FROM daily_ohlcv o JOIN perpetual_futures pf ON pf.id = o.coin_id WHERE symbol LIKE '%USDT' ORDER BY o.opentime ASC")
    perps = CUR.fetchall()
    df = pd.DataFrame(perps)
    df.columns = ["symbol", "opentime", "close"]
    df['close'] = df['close'].astype(float)
    df = df.pivot(columns="symbol", values="close", index="opentime")
    return df


def emaforecast(closes, index, fast, slow):
    df = pd.DataFrame()
    df.index = index
    df['close'] = closes
    df['returns'] = df['close'].pct_change()
    df['next_day_returns'] = df['returns'].shift(-1)

    df['ema_fast'] = df.ta.ema(fast)
    df['ema_slow'] = df.ta.ema(slow)

    df['forecast'] = df['ema_fast'] - df['ema_slow']
    df['adjusted_forecast'] = (df['forecast'] * 10) / (df['forecast'].abs().expanding().mean())
    df['clamped_forecast'] = df['adjusted_forecast'].clip(upper=20, lower=-20)

    return df
