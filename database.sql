CREATE TABLE IF NOT EXISTS perpetual_futures(
       id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
       symbol TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daily_ohlcv(
       coin_id BIGINT references perpetual_futures(id),
       opentime BIGINT NOT NULL,
       openprice TEXT NOT NULL,
       highprice TEXT NOT NULL,
       lowprice TEXT NOT NULL,
       closeprice TEXT NOT NULL,
       volume TEXT NOT NULL
);
