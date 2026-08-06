# sol-guard — When and How to Use

> Safety-first rule for every Solana token interaction: **acknowledge,
> check, then act** — send a short "Checking…" message, run the check, and
> show the user the verdict.

---

## 🔒 HARD RULE #1 — Acknowledge BEFORE you check. Never skip it.

**For ANY message about a token's safety, risk, trustworthiness, or whether
to buy/swap/accept/hold it, the following order is MANDATORY — not
optional, not "best practice", not a suggestion:**

1. **First**, immediately send this exact message:

   ```text
   🔍 Checking token risk… (usually takes a few seconds)
   ```

2. **Only after that message is sent**, call the `sol-guard` tool.
3. **Then** surface the full risk report to the user.

This rule applies on **every** token-risk request, without exception:

- "Is this token safe?"
- "Check this mint: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`"
- "Should I buy / swap this token?"
- "Can I trust this token?"
- "Someone wants to pay me in this token — what do you think?"
- "How risky is this token / project?"
- Any message that is clearly about token risk.

### The exact sequence you MUST follow

```text
User:  Is this token safe? EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
Agent: 🔍 Checking token risk… (usually takes a few seconds)     ← step 1, ALWAYS FIRST
       (calls sol-guard with that mint)                           ← step 2, ONLY AFTER
Agent: 🟡 AMBER — Medium Risk
       Score: 65/100 · Confidence: 95%
       • Mint authority still active
       Advice: Proceed with caution — this token has active admin controls…
                                                                   ← step 3, verdict verbatim
```

### NEVER do this (these are violations)

```text
✗ User:  Is this token safe? <mint>
  Agent: (calls sol-guard, no "Checking…" message sent)
         🟢 GREEN — Low Risk
  → VIOLATION: the acknowledgment was skipped.

✗ User:  Is this token safe? <mint>
  Agent: 🔴 RED — High Risk. Do not buy or swap this token.
  → VIOLATION: final verdict sent with NO "Checking…" message and NO tool call.

✗ User:  Is this token safe? <mint>
  Agent: Let me check that for you right away. One moment please while I
         look up the token details on-chain and analyze the results...
  → VIOLATION: long free-form preamble instead of the exact short
    "🔍 Checking token risk…" message. Use the exact text, not a paraphrase.
```

**Why this rule exists and why it is non-negotiable:** the on-chain RPC
check can take seconds, and the user must never stare at silence after
asking about their money. The acknowledgment is the confirmation that the
request was received and the check is running.

**Never** skip it because you think the check "might be fast". **Never**
reply with the final verdict unless the acknowledgment was already sent
first. If you realize you are about to answer without having sent the
"Checking…" message, **stop and send it before doing anything else.**

---

## What it does

`sol-guard` is a read-only (T0) tool plugin that assesses the on-chain risk
profile of a Solana **token mint**: mint/freeze authority status, dangerous
Token-2022 extensions (permanent delegate, transfer hooks, transfer fees,
non-transferable, pausable), initialization state, and holder
concentration (what share of supply the top-10 accounts control). It
returns a chat-ready verdict — emoji + score/100 + confidence %, up to 5
short reasons, and one actionable `Advice:` line.

## When to call it — mandatory gate

Always call `sol-guard` on a token **before** you do any of these:

- Swapping into, out of, or buying a token
- Accepting payment in an unfamiliar token
- Recommending a token, an address, or a project's token
- Investigating a token mentioned by the user, a site, or a message
- Holding any token in a portfolio/wallet plan

If a token will be swapped, transferred, recommended, or trusted, the check
is not optional.

## User messages that should trigger it

- "Should I buy this token?"
- "Is this token safe: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`?"
- "Someone wants to pay me in <token>. Can I trust it?"
- "Swap my SOL for <mint address>"
- "How risky is <project>?"
- "Is this a scam? <address or token name>"
- "What's the risk of holding <token>?"

If the message is about buying, selling, transferring, trusting, or valuing
a specific token, run the check. When in doubt, run it.

## What to do with the result

1. **Acknowledge first** — the `🔍 Checking token risk…` message goes out
   *before* the tool call (Hard Rule #1 above). Never after, never instead
   of a reply.
2. **Surface the verdict verbatim.** Quote the full output block to the
   user — the emoji verdict line, score, confidence, and the `Advice:`
   line. Do not bury it, paraphrase it away, or dump raw data.
3. **Act on the verdict:**
   - 🔴 **RED** — Do **not** proceed with the swap/transfer/recommendation.
     Tell the user it is high-risk, quote the `Advice:` line, and explain
     what would change the verdict (e.g. "if the issuer renounces mint
     authority and removes the permanent delegate").
   - 🟡 **AMBER** — Proceed **only with the user's explicit, informed
     consent**. State the active admin controls before doing anything.
   - 🟢 **GREEN** — Safe to proceed, but still note
     "Still do your own research." — the check is not a liquidity or market
     analysis.
4. **One check per token per conversation** is enough. Don't re-run for the
   same mint unless the user asks or the context changes.

## Good vs bad usage

**Good:**
- User: "Is this token safe to buy: `<mint>`?"
  Agent: sends `🔍 Checking token risk… (usually takes a few seconds)`,
  calls `sol-guard`, then replies with the full verdict block, e.g.
  "🔴 RED — High Risk … Advice: Do not buy or swap this token…", and
  refuses to recommend a swap.
- User: "Swap my SOL for `<mint>`."
  Agent: sends `🔍 Checking token risk…`, calls `sol-guard` **first**,
  shares the verdict, and only executes the swap if the verdict allows it
  and the user confirms.

**Bad (violations of Hard Rule #1):**
- Calling `sol-guard` without first sending the `🔍 Checking token risk…`
  reassurance message — the user should never wait in silence.
- Replying with the final verdict without ever having sent the "Checking…"
  message.
- Replacing the exact "Checking…" message with a long free-form preamble.
- Recommending or swapping a token without ever calling `sol-guard`.
- Calling `sol-guard` but replying only "it's a bit risky" — the user needs
  the verdict, score, and advice.
- Sending the user the raw JSON-RPC response or internal findings dump.
- Re-running the tool on the same mint repeatedly in one conversation.
- Calling it on a non-mint address (e.g. a wallet address or SOL itself) —
  it validates input and rejects non-mints; don't retry with variations.

## When NOT to call it

- The interaction is not about a token (e.g. generic Solana questions,
  ecosystem news).
- The address is a wallet/account, not a token mint.
- The user is asking about SOL itself — SOL is not an SPL mint.

## Output format reminder

The tool returns a ready-to-paste chat block:

```text
🔴 RED — High Risk
Score: 20/100 · Confidence: 95%

• Top 10 holders control 68% of supply
• Permanent delegate present
• Mint authority still active

Advice: Do not buy or swap this token. Wait until the dangerous controls are renounced or removed.
```

Copy it to the user as-is. Never reformat it into JSON or tables.

## Error handling

If the tool errors (invalid address, RPC unreachable), the error message is
already user-safe. Report it simply ("that doesn't look like a valid mint
address" / "couldn't reach the RPC endpoint") and do not invent a verdict.
If you sent the `🔍 Checking token risk…` message and the tool then errors,
reply with the safe error message — do not leave the user hanging.
