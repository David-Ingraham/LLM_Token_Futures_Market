import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { getAssociatedTokenAddressSync } from "@llm-token-futures/sdk";
import { PublicKey, Transaction } from "@solana/web3.js";
import { useCallback, useEffect, useState } from "react";
import {
  fetchMarket,
  fetchPosition,
  haltTrading,
  openPosition,
  postSettlementPrice,
  positionPda,
  settlePosition,
  Side,
  MarketStatus,
} from "@llm-token-futures/sdk";
import {
  requiredMarginMicro,
  settlementPnlMicro,
} from "@llm-token-futures/settlement";
import { loadDevnetConfig, type DevnetConfig } from "./devnetConfig";

function microToUsd(micro: number | bigint): string {
  const n = typeof micro === "bigint" ? Number(micro) : micro;
  return (n / 1_000_000).toFixed(2);
}

export default function App() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const [cfg, setCfg] = useState<DevnetConfig | null>(null);
  const [market, setMarket] = useState<Awaited<ReturnType<typeof fetchMarket>>>(null);
  const [position, setPosition] = useState<Awaited<ReturnType<typeof fetchPosition>>>(null);
  const [contracts, setContracts] = useState("1");
  const [side, setSide] = useState<number>(Side.Long);
  const [settlePrice, setSettlePrice] = useState("27000000");
  const [log, setLog] = useState("");

  const append = (msg: string) => setLog((l) => `${new Date().toISOString()} ${msg}\n${l}`);

  useEffect(() => {
    loadDevnetConfig().then(setCfg);
  }, []);

  const refresh = useCallback(async () => {
    if (!cfg) return;
    const m = await fetchMarket(connection, new PublicKey(cfg.market));
    setMarket(m);
    if (wallet.publicKey) {
      const [posPk] = positionPda(new PublicKey(cfg.market), wallet.publicKey);
      const p = await fetchPosition(connection, posPk);
      setPosition(p);
    }
  }, [cfg, connection, wallet.publicKey]);

  useEffect(() => {
    refresh();
    const id = setInterval(refresh, 8000);
    return () => clearInterval(id);
  }, [refresh]);

  async function runTx(label: string, tx: Transaction) {
    if (!wallet.publicKey || !wallet.signTransaction) {
      append("Connect wallet");
      return;
    }
    tx.feePayer = wallet.publicKey;
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    const signed = await wallet.signTransaction(tx);
    const sig = await connection.sendRawTransaction(signed.serialize());
    await connection.confirmTransaction({ signature: sig, blockhash, lastValidBlockHeight });
    append(`${label} ok: ${sig}`);
    await refresh();
  }

  async function onOpen() {
    if (!cfg || !wallet.publicKey) return;
    const marketPk = new PublicKey(cfg.market);
    const mint = new PublicKey(cfg.mint);
    const [posPk] = positionPda(marketPk, wallet.publicKey);
    const ata = getAssociatedTokenAddressSync(mint, wallet.publicKey);
    const ix = openPosition(
      {
        owner: wallet.publicKey,
        market: marketPk,
        position: posPk,
        ownerTokenAccount: ata,
        vault: new PublicKey(cfg.vault),
      },
      side,
      BigInt(contracts),
    );
    await runTx("open_position", new Transaction().add(ix));
  }

  async function onHalt() {
    if (!cfg || !wallet.publicKey) return;
    await runTx(
      "halt_trading",
      new Transaction().add(haltTrading(new PublicKey(cfg.market), wallet.publicKey)),
    );
  }

  async function onPostPrice() {
    if (!cfg || !wallet.publicKey) return;
    await runTx(
      "post_settlement_price",
      new Transaction().add(
        postSettlementPrice(
          wallet.publicKey,
          new PublicKey(cfg.market),
          BigInt(settlePrice),
        ),
      ),
    );
  }

  async function onSettle() {
    if (!cfg || !wallet.publicKey) return;
    const marketPk = new PublicKey(cfg.market);
    const mint = new PublicKey(cfg.mint);
    const [posPk] = positionPda(marketPk, wallet.publicKey);
    const ata = getAssociatedTokenAddressSync(mint, wallet.publicKey);
    await runTx(
      "settle_position",
      new Transaction().add(
        settlePosition({
          owner: wallet.publicKey,
          market: marketPk,
          position: posPk,
          ownerTokenAccount: ata,
          vault: new PublicKey(cfg.vault),
        }),
      ),
    );
  }

  const previewPnl =
    market && position && !position.settled && market.settlementPriceMicro > 0n
      ? settlementPnlMicro(
          position.side === Side.Long ? "long" : "short",
          Number(position.contracts),
          Number(market.entryPriceMicro),
          Number(market.settlementPriceMicro),
        )
      : null;

  const marginPreview =
    market && contracts
      ? requiredMarginMicro(
          side === Side.Long ? "long" : "short",
          Number(contracts),
          Number(market.entryPriceMicro),
          {
            maxSettlementPriceMicro: Number(market.maxSettlementPriceMicro),
            contractMtok: 1,
            feeBps: 0,
          },
        )
      : null;

  return (
    <main>
      <h1>Opus 4.7 output index futures (devnet)</h1>
      <WalletMultiButton />
      <p className="status">
        {cfg
          ? `Market ${cfg.market.slice(0, 8)}… · program ${cfg.programId.slice(0, 8)}…`
          : "Run npm run deploy:devnet and reload (needs app/public/devnet.json)"}
      </p>

      <section>
        <h2>1. Market state</h2>
        {market ? (
          <div className="mono">
            <div>Status: {statusLabel(market.status)}</div>
            <div>Entry F0: ${microToUsd(market.entryPriceMicro)} / MTok</div>
            <div>Settlement FT: {market.settlementPriceMicro > 0n ? `$${microToUsd(market.settlementPriceMicro)}` : "not posted"}</div>
            <div>Cutoff: {new Date(Number(market.tradeCutoffTs) * 1000).toLocaleString()}</div>
            <div>Expiry: {new Date(Number(market.expiryTs) * 1000).toLocaleString()}</div>
          </div>
        ) : (
          <p className="status">No market loaded</p>
        )}
      </section>

      <section>
        <h2>2. Open position</h2>
        <label>
          Side
          <select value={side} onChange={(e) => setSide(Number(e.target.value))}>
            <option value={Side.Long}>Long</option>
            <option value={Side.Short}>Short</option>
          </select>
        </label>
        <label>
          Contracts
          <input value={contracts} onChange={(e) => setContracts(e.target.value)} />
        </label>
        {marginPreview != null && (
          <p className="status">Required margin: ${microToUsd(marginPreview)} USDC</p>
        )}
        <button onClick={onOpen} disabled={!wallet.connected || !cfg}>
          Deposit margin and open
        </button>
      </section>

      <section>
        <h2>3. Your position</h2>
        {position && position.contracts > 0n && !position.settled ? (
          <div className="mono">
            <div>{position.side === Side.Long ? "LONG" : "SHORT"} × {position.contracts.toString()}</div>
            <div>Locked margin: ${microToUsd(position.lockedMargin)}</div>
            {previewPnl != null && <div>Est. PnL at FT: ${microToUsd(previewPnl)}</div>}
          </div>
        ) : (
          <p className="status">No open position</p>
        )}
      </section>

      <section>
        <h2>4. Lifecycle (oracle / settle)</h2>
        <p className="status">After cutoff: halt. After expiry: post price (oracle wallet). Then settle.</p>
        <button onClick={onHalt} disabled={!wallet.connected || !cfg}>
          Halt trading
        </button>
        <label>
          Settlement price (micro-USD / MTok)
          <input value={settlePrice} onChange={(e) => setSettlePrice(e.target.value)} />
        </label>
        <button onClick={onPostPrice} disabled={!wallet.connected || !cfg}>
          Post settlement price
        </button>
        <button onClick={onSettle} disabled={!wallet.connected || !cfg}>
          Settle my position
        </button>
      </section>

      <section>
        <h2>Log</h2>
        <div className="log">{log || "—"}</div>
      </section>
    </main>
  );
}

function statusLabel(s: number): string {
  if (s === MarketStatus.Open) return "Open";
  if (s === MarketStatus.Halted) return "Halted";
  if (s === MarketStatus.PricePosted) return "PricePosted";
  return `Unknown(${s})`;
}
