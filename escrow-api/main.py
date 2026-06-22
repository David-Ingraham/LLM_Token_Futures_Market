from fastapi import FastAPI, HTTPException

app = FastAPI()

escrow = 0.0
markets = {}       # market_id -> {price, expiry}
long_q = {}          # market_id -> [orders waiting]
short_q = {}         # market_id -> [orders waiting]
matched = []         # paired bets
balances = {}        # user -> dollars after settle


def pay(user, amount):
    balances[user] = balances.get(user, 0.0) + amount


def queue_for(side, market_id):
    q = long_q if side == "long" else short_q
    return q.setdefault(market_id, [])


@app.post("/markets")
def create_market(market_id: str, price: float, expiry: str):
    if market_id in markets:
        raise HTTPException(400, "market already exists")
    if price <= 0:
        raise HTTPException(400, "price must be positive")
    markets[market_id] = {"price": price, "expiry": expiry}
    return {"market_id": market_id, **markets[market_id]}


@app.post("/order")
def post_order(market_id: str, user: str, side: str, contracts: int, money: float):
    global escrow
    if market_id not in markets:
        raise HTTPException(404, "market not found")
    if side not in ("long", "short"):
        raise HTTPException(400, "side must be long or short")
    if contracts <= 0 or money <= 0:
        raise HTTPException(400, "contracts and money must be positive")

    price = markets[market_id]["price"]
    order = {"user": user, "contracts": contracts, "money": money}
    escrow += money

    opposite = queue_for("short" if side == "long" else "long", market_id)
    own = queue_for(side, market_id)
    new_pairs = []

    while order["contracts"] > 0 and opposite:
        other = opposite[0]
        fill = min(order["contracts"], other["contracts"])
        margin = fill * price

        pair = {
            "market_id": market_id,
            "long": order["user"] if side == "long" else other["user"],
            "short": other["user"] if side == "long" else order["user"],
            "contracts": fill,
            "price": price,
            "margin": margin,
        }
        matched.append(pair)
        new_pairs.append(pair)

        order["contracts"] -= fill
        other["contracts"] -= fill
        other["money"] -= fill * price
        if other["contracts"] == 0:
            opposite.pop(0)

    if order["contracts"] > 0:
        order["money"] = order["contracts"] * price
        own.append(order)

    return {"escrow": escrow, "matched": new_pairs, "queued": order["contracts"] > 0}


@app.get("/status")
def status():
    return {
        "escrow": escrow,
        "markets": markets,
        "long_q": long_q,
        "short_q": short_q,
        "matched": matched,
        "balances": balances,
    }


@app.post("/settle")
def settle(market_id: str, settle_price: float):
    global escrow
    if market_id not in markets:
        raise HTTPException(404, "market not found")

    payouts = []
    remaining = []

    for pair in matched:
        if pair["market_id"] != market_id:
            remaining.append(pair)
            continue

        delta = settle_price - pair["price"]
        long_pay = pair["margin"] + delta * pair["contracts"]
        short_pay = pair["margin"] - delta * pair["contracts"]

        pay(pair["long"], long_pay)
        pay(pair["short"], short_pay)
        escrow -= long_pay + short_pay

        payouts.append(
            {
                "long": pair["long"],
                "short": pair["short"],
                "contracts": pair["contracts"],
                "long_pay": long_pay,
                "short_pay": short_pay,
            }
        )

    matched[:] = remaining
    del markets[market_id]

    return {"escrow": escrow, "payouts": payouts, "balances": balances}
